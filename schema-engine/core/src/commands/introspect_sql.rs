/// TODO(sr): Fix this `as` use
use schema_connector::{IntrospectSqlQueryInput, IntrospectSqlResult as ConnectorIntrospectSqlResult, SchemaConnector};

/// Params type for the introspectSql method.
#[derive(Debug)]
pub struct IntrospectSqlParams {
    /// The database URL.
    pub url: String,
    /// SQL queries to introspect.
    pub queries: Vec<SqlQueryInput>,
}

/// Input for a single SQL query.
#[derive(Debug)]
pub struct SqlQueryInput {
    /// The name of the query.
    pub name: String,
    /// The source SQL.
    pub source: String,
}

/// Result type for the introspectSql method.
pub struct IntrospectSqlResult {
    /// The introspected queries.
    pub queries: Vec<SqlQueryOutput>,
}



/// Output for a single SQL query.
#[derive(Debug)]
pub struct SqlQueryOutput {
    /// The name of the query.
    pub name: String,
    /// The source SQL.
    pub source: String,
    /// Optional documentation.
    pub documentation: Option<String>,
    /// Query parameters.
    pub parameters: Vec<SqlQueryParameterOutput>,
    /// Query result columns.
    pub result_columns: Vec<SqlQueryColumnOutput>,
}

/// Information about a SQL query parameter.
#[derive(Debug)]
pub struct SqlQueryParameterOutput {
    /// Parameter name.
    pub name: String,
    /// Parameter type.
    pub typ: String,
    /// Optional documentation.
    pub documentation: Option<String>,
    /// Whether the parameter is nullable.
    pub nullable: bool,
}

/// Information about a SQL query result column.
#[derive(Debug)]
pub struct SqlQueryColumnOutput {
    /// Column name.
    pub name: String,
    /// Column type.
    pub typ: String,
    /// Whether the column is nullable.
    pub nullable: bool,
}

///
pub async fn introspect_sql(
    input: IntrospectSqlParams,
    connector: &mut dyn SchemaConnector,
) -> crate::CoreResult<ConnectorIntrospectSqlResult> {
    let queries: Vec<_> = input
        .queries
        .into_iter()
        .map(|q| IntrospectSqlQueryInput {
            name: q.name,
            source: q.source,
        })
        .collect();

    let mut parsed_queries = Vec::with_capacity(queries.len());

    for q in queries {
        let parsed_query = connector.introspect_sql(q).await?;

        parsed_queries.push(parsed_query);
    }

    Ok(ConnectorIntrospectSqlResult {
        queries: parsed_queries,
    })
}
