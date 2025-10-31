pub mod completions;

#[cfg(feature = "mysql")]
pub use mysql_datamodel_connector::MySqlType;
#[cfg(feature = "postgresql")]
pub use postgres_datamodel_connector::{KnownPostgresType, PostgresDatasourceProperties, PostgresType};

mod capabilities_support;
#[cfg(feature = "mysql")]
mod mysql_datamodel_connector;
mod native_type_definition;
#[cfg(feature = "postgresql")]
mod postgres_datamodel_connector;
#[cfg(feature = "sqlite")]
mod sqlite_datamodel_connector;
mod utils;
pub use capabilities_support::{can_have_capability, can_support_relation_load_strategy, has_capability};

use crate::ConnectorRegistry;

#[cfg(feature = "postgresql")]
pub const POSTGRES: &'static dyn crate::datamodel_connector::Connector =
    &postgres_datamodel_connector::PostgresDatamodelConnector;
#[cfg(feature = "mysql")]
pub const MYSQL: &'static dyn crate::datamodel_connector::Connector =
    &mysql_datamodel_connector::MySqlDatamodelConnector;
#[cfg(feature = "sqlite")]
pub const SQLITE: &'static dyn crate::datamodel_connector::Connector =
    &sqlite_datamodel_connector::SqliteDatamodelConnector;

pub static BUILTIN_CONNECTORS: ConnectorRegistry<'static> = &[
    #[cfg(feature = "postgresql")]
    POSTGRES,
    #[cfg(feature = "mysql")]
    MYSQL,
    #[cfg(feature = "sqlite")]
    SQLITE,
];
