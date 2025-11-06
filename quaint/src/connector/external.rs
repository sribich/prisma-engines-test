use std::{str::FromStr, sync::Arc};

use async_trait::async_trait;

use super::{SqlFamily, TransactionCapable};

#[derive(Debug, Clone)]
pub struct ExternalConnectionInfo {
    // TODO: `sql_family` doesn't exist in TypeScript's `ConnectionInfo` type.
    pub sql_family: SqlFamily,
    pub schema_name: Option<String>,
    pub max_bind_values: Option<usize>,
    pub supports_relation_joins: bool,
}

impl ExternalConnectionInfo {
    pub fn new(
        sql_family: SqlFamily,
        schema_name: Option<String>,
        max_bind_values: Option<usize>,
        supports_relation_joins: bool,
    ) -> Self {
        ExternalConnectionInfo {
            sql_family,
            schema_name,
            max_bind_values,
            supports_relation_joins,
        }
    }
}

#[async_trait]
pub trait ExternalConnector: TransactionCapable {
    async fn get_connection_info(&self) -> crate::Result<ExternalConnectionInfo>;
    async fn execute_script(&self, script: &str) -> crate::Result<()>;
    async fn dispose(&self) -> crate::Result<()>;

    /// Returns a reference to self as an ExternalConnector.
    fn as_external_connector(&self) -> Option<&dyn ExternalConnector>
    where
        Self: Sized,
    {
        Some(self)
    }
}

#[async_trait]
pub trait ExternalConnectorFactory: Send + Sync {
    async fn connect(&self) -> crate::Result<Arc<dyn ExternalConnector>>;
    async fn connect_to_shadow_db(&self) -> Option<crate::Result<Arc<dyn ExternalConnector>>>;
}
