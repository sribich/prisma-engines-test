mod json_adapter;

pub use json_adapter::*;
use serde::{Deserialize, Serialize};
use telemetry::TraceParent;

use crate::{
    ConnectorTag, ConnectorVersion, ENGINE_PROTOCOL, QueryResult, RenderedDatamodel, TestError, TestLogCapture,
    TestResult,
};
use colored::Colorize;
use prisma_metrics::MetricRegistry;
use query_core::{
    QueryExecutor, TransactionOptions, TxId,
    protocol::EngineProtocol,
    relation_load_strategy,
    schema::{self, QuerySchemaRef},
};
use request_handlers::{
    BatchTransactionOption, ConnectorKind, GraphqlBody, JsonBatchQuery, JsonBody, JsonSingleQuery, MultiQuery,
    RequestBody, RequestHandler,
};
use serde_json::json;
use std::{
    fmt::Display,
    sync::{Arc, atomic::AtomicUsize},
};

pub type TxResult = Result<(), user_facing_errors::Error>;

pub(crate) type Executor = Box<dyn QueryExecutor + Send + Sync>;

#[derive(Deserialize, Debug)]
struct Empty {}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum TransactionEndResponse {
    Error(user_facing_errors::Error),
    Ok(Empty),
}

impl From<TransactionEndResponse> for TxResult {
    fn from(value: TransactionEndResponse) -> Self {
        match value {
            TransactionEndResponse::Ok(_) => Ok(()),
            TransactionEndResponse::Error(error) => Err(error),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum StartTransactionResponse {
    Ok { id: String },
    Error(user_facing_errors::Error),
}

pub enum RunnerExecutor {
    // Builtin is a runner that uses the query engine in-process, issuing queries against a
    // `core::InterpretingExecutor` that uses the particular connector under test in the test suite.
    Builtin(Executor),
}

/// Direct engine runner.
pub struct Runner {
    executor: RunnerExecutor,
    query_schema: QuerySchemaRef,
    version: ConnectorVersion,
    connector_tag: ConnectorTag,
    connection_url: String,
    current_tx_id: Option<TxId>,
    metrics: MetricRegistry,
    protocol: EngineProtocol,
    log_capture: TestLogCapture,
    local_max_bind_values: Option<usize>,
}

impl Runner {
    pub(crate) fn schema_id(&self) -> Option<usize> {
        match &self.executor {
            RunnerExecutor::Builtin(_) => None,
        }
    }

    pub fn prisma_dml(&self) -> &str {
        self.query_schema.internal_data_model.schema.db.source_assert_single()
    }

    pub fn max_bind_values(&self) -> Option<usize> {
        self.local_max_bind_values
            .or_else(|| self.connector_version().max_bind_values())
    }

    pub async fn load(
        datamodel: &RenderedDatamodel,
        db_schemas: &[&str],
        connector_version: ConnectorVersion,
        connector_tag: ConnectorTag,
        override_local_max_bind_values: Option<usize>,
        metrics: MetricRegistry,
        log_capture: TestLogCapture,
    ) -> TestResult<Self> {
        let protocol = EngineProtocol::from(&ENGINE_PROTOCOL.to_string());
        let schema = psl::parse_schema_without_extensions(&datamodel.schema).unwrap();
        let datasource = schema.configuration.datasources.first().unwrap();

        let (executor, db_version) = match crate::CONFIG.with_driver_adapter() {
            Some(with_driver_adapter) => {
                // TODO(sr): Remove this and driver_adapter
                panic!("Driver adapter not supported");
            }
            None => {
                qe_setup::setup(datamodel.url.clone(), &datamodel.schema, db_schemas).await?;

                let query_executor = request_handlers::load_executor(
                    ConnectorKind::Rust {
                        url: datamodel.url.clone(),
                        datasource,
                    },
                    schema.configuration.preview_features(),
                    true,
                )
                .await?;
                let connector = query_executor.primary_connector();
                let conn = connector.get_connection().await.unwrap();
                let database_version = conn.version().await;
                let executor = RunnerExecutor::Builtin(query_executor);

                (executor, database_version)
            }
        };

        // If `override_local_max_bind_values` is provided, use that.
        // Otherwise, if the external process has provided an `init_result`, use `init_result.max_bind_values`.
        // Otherwise, use the connector's (Wasm-aware) default.
        //
        // Note: Use `override_local_max_bind_values` only for local testing purposes.
        // If a feature requires a specific `max_bind_values` value for a Driver Adapter, it should be set in the
        // TypeScript Driver Adapter implementation itself.
        let local_max_bind_values = override_local_max_bind_values;

        let query_schema = schema::build(Arc::new(schema), true).with_db_version_supports_join_strategy(
            relation_load_strategy::db_version_supports_joins_strategy(db_version)?,
        );

        Ok(Self {
            version: connector_version,
            executor,
            query_schema: Arc::new(query_schema),
            connector_tag,
            connection_url: datamodel.url.clone(),
            current_tx_id: None,
            metrics,
            protocol,
            log_capture,
            local_max_bind_values,
        })
    }

    pub async fn query(&self, query: impl Into<String>) -> TestResult<QueryResult> {
        self.query_with_params(self.current_tx_id.as_ref(), None, query).await
    }

    pub async fn query_in_tx(&self, tx_id: &TxId, query: impl Into<String>) -> TestResult<QueryResult> {
        self.query_with_params(Some(tx_id), None, query).await
    }

    pub async fn query_with_traceparent(
        &self,
        traceparent: TraceParent,
        query: impl Into<String>,
    ) -> TestResult<QueryResult> {
        self.query_with_params(None, Some(traceparent), query).await
    }

    async fn query_with_params<T>(
        &self,
        tx_id: Option<&TxId>,
        traceparent: Option<TraceParent>,
        query: T,
    ) -> TestResult<QueryResult>
    where
        T: Into<String>,
    {
        let query = query.into();

        let executor = match &self.executor {
            RunnerExecutor::Builtin(e) => e,
        };

        tracing::info!("Querying: {}", query);

        let handler = RequestHandler::new(&**executor, &self.query_schema, self.protocol);

        let request_body = match self.protocol {
            EngineProtocol::Json => {
                // Translate the GraphQL query to JSON
                let json_query = JsonRequest::from_graphql(&query, self.query_schema())?;
                println!("{}", serde_json::to_string_pretty(&json_query).unwrap().green());

                RequestBody::Json(JsonBody::Single(json_query))
            }
            EngineProtocol::Graphql => {
                println!("{}", query.bright_green());

                RequestBody::Graphql(GraphqlBody::Single(query.into()))
            }
        };

        let response = handler.handle(request_body, tx_id.cloned(), traceparent).await;

        let result: QueryResult = match self.protocol {
            EngineProtocol::Json => JsonResponse::from_graphql(response).into(),
            EngineProtocol::Graphql => response.into(),
        };

        if result.failed() {
            tracing::debug!("Response: {}", result.to_string().red());
        } else {
            tracing::debug!("Response: {}", result.to_string().green());
        }

        Ok(result)
    }

    pub async fn query_json(&self, query: impl Display) -> TestResult<QueryResult> {
        let query = query.to_string();

        tracing::debug!("Querying: {}", query.clone().green());

        println!("{}", query.bright_green());
        let query: serde_json::Value = serde_json::from_str(&query).unwrap();

        let executor = match &self.executor {
            RunnerExecutor::Builtin(e) => e,
        };

        let handler = RequestHandler::new(&**executor, &self.query_schema, EngineProtocol::Json);

        let serialized_query: JsonSingleQuery = serde_json::from_value(query).unwrap();
        let request_body = RequestBody::Json(JsonBody::Single(serialized_query));

        let result: QueryResult = handler
            .handle(request_body, self.current_tx_id.clone(), None)
            .await
            .into();

        if result.failed() {
            tracing::debug!("Response: {}", result.to_string().red());
        } else {
            tracing::debug!("Response: {}", result.to_string().green());
        }

        Ok(result)
    }

    pub async fn raw_execute<T>(&self, query: T) -> TestResult<()>
    where
        T: Into<String>,
    {
        let query = query.into();
        tracing::debug!("Raw execute: {}", query.clone().green());

        self.connector_tag.raw_execute(&query, &self.connection_url).await?;

        Ok(())
    }

    pub async fn batch_json(
        &self,
        queries: Vec<String>,
        transaction: bool,
        isolation_level: Option<String>,
    ) -> TestResult<crate::QueryResult> {
        let executor = match &self.executor {
            RunnerExecutor::Builtin(e) => e,
        };

        let handler = RequestHandler::new(&**executor, &self.query_schema, self.protocol);
        let body = RequestBody::Json(JsonBody::Batch(JsonBatchQuery {
            batch: queries
                .into_iter()
                .map(|q| serde_json::from_str::<JsonSingleQuery>(&q).unwrap())
                .collect(),
            transaction: transaction.then_some(BatchTransactionOption { isolation_level }),
        }));

        let res = handler.handle(body, self.current_tx_id.clone(), None).await;

        Ok(res.into())
    }

    pub async fn batch(
        &self,
        queries: Vec<String>,
        transaction: bool,
        isolation_level: Option<String>,
    ) -> TestResult<crate::QueryResult> {
        let executor = match &self.executor {
            RunnerExecutor::Builtin(e) => e,
        };

        let handler = RequestHandler::new(&**executor, &self.query_schema, self.protocol);
        let body = match self.protocol {
            EngineProtocol::Json => {
                // Translate the GraphQL query to JSON
                let batch = queries
                    .into_iter()
                    .map(|query| JsonRequest::from_graphql(&query, self.query_schema()))
                    .collect::<TestResult<Vec<_>>>()
                    .unwrap();
                let transaction_opts = match transaction {
                    true => Some(BatchTransactionOption { isolation_level }),
                    false => None,
                };

                println!("{}", serde_json::to_string_pretty(&batch).unwrap().green());

                RequestBody::Json(JsonBody::Batch(JsonBatchQuery {
                    batch,
                    transaction: transaction_opts,
                }))
            }
            EngineProtocol::Graphql => RequestBody::Graphql(GraphqlBody::Multi(MultiQuery::new(
                queries.into_iter().map(Into::into).collect(),
                transaction,
                isolation_level,
            ))),
        };

        let res = handler.handle(body, self.current_tx_id.clone(), None).await;

        match self.protocol {
            EngineProtocol::Json => Ok(JsonResponse::from_graphql(res).into()),
            EngineProtocol::Graphql => Ok(res.into()),
        }
    }

    pub async fn start_tx(
        &self,
        max_acquisition_millis: u64,
        valid_for_millis: u64,
        isolation_level: Option<String>,
    ) -> TestResult<TxId> {
        let tx_opts = TransactionOptions::new(max_acquisition_millis, valid_for_millis, isolation_level);
        match &self.executor {
            RunnerExecutor::Builtin(executor) => {
                let id = executor
                    .start_tx(self.query_schema.clone(), self.protocol, tx_opts)
                    .await?;
                Ok(id)
            }
        }
    }

    pub async fn commit_tx(&self, tx_id: TxId) -> TestResult<TxResult> {
        match &self.executor {
            RunnerExecutor::Builtin(executor) => {
                let res = executor.commit_tx(tx_id).await;

                if let Err(error) = res {
                    Ok(Err(error.into()))
                } else {
                    Ok(Ok(()))
                }
            }
        }
    }

    pub async fn rollback_tx(&self, tx_id: TxId) -> TestResult<TxResult> {
        match &self.executor {
            RunnerExecutor::Builtin(executor) => {
                let res = executor.rollback_tx(tx_id).await;

                if let Err(error) = res {
                    Ok(Err(error.into()))
                } else {
                    Ok(Ok(()))
                }
            }
        }
    }

    pub fn connector(&self) -> &crate::ConnectorTag {
        &self.connector_tag
    }

    pub fn set_active_tx(&mut self, tx_id: query_core::TxId) {
        self.current_tx_id = Some(tx_id);
    }

    pub fn clear_active_tx(&mut self) {
        self.current_tx_id = None;
    }

    pub fn get_metrics(&self) -> MetricRegistry {
        self.metrics.clone()
    }

    pub fn query_schema(&self) -> &QuerySchemaRef {
        &self.query_schema
    }

    pub async fn get_logs(&mut self) -> Vec<String> {
        let mut logs = self.log_capture.get_logs().await;
        match &self.executor {
            RunnerExecutor::Builtin(_) => logs,
        }
    }

    pub async fn clear_logs(&mut self) {
        self.log_capture.clear_logs().await
    }

    pub fn connector_version(&self) -> &ConnectorVersion {
        &self.version
    }

    pub fn protocol(&self) -> EngineProtocol {
        self.protocol
    }
}
