use std::{str::FromStr, sync::Arc};

use async_trait::async_trait;

use super::{SqlFamily, TransactionCapable};

#[async_trait]
pub trait ExternalConnector: TransactionCapable {
    async fn dispose(&self) -> crate::Result<()>;
}

#[async_trait]
pub trait ExternalConnectorFactory: Send + Sync {
    async fn connect(&self) -> crate::Result<Arc<dyn ExternalConnector>>;
    async fn connect_to_shadow_db(&self) -> Option<crate::Result<Arc<dyn ExternalConnector>>>;
}
