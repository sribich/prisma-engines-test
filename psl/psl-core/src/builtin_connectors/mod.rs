pub mod completions;
mod capabilities_support;

mod native_type_definition;
#[cfg(feature = "mysql")]
mod mysql;
#[cfg(feature = "postgresql")]
mod postgres;
#[cfg(feature = "sqlite")]
mod sqlite;

mod utils;

#[cfg(feature = "mysql")]
pub use mysql::MySqlType;
#[cfg(feature = "postgresql")]
pub use postgres::{KnownPostgresType, PostgresDatasourceProperties, PostgresType, SequenceFunction};


pub use capabilities_support::{can_have_capability, can_support_relation_load_strategy, has_capability};

use crate::ConnectorRegistry;

#[cfg(feature = "postgresql")]
pub const POSTGRES: &'static dyn crate::datamodel_connector::Connector =
    &postgres::PostgresDatamodelConnector;
#[cfg(feature = "mysql")]
pub const MYSQL: &'static dyn crate::datamodel_connector::Connector =
    &mysql::MySqlDatamodelConnector;
#[cfg(feature = "sqlite")]
pub const SQLITE: &'static dyn crate::datamodel_connector::Connector =
    &sqlite::SqliteDatamodelConnector;

pub static BUILTIN_CONNECTORS: ConnectorRegistry<'static> = &[
    #[cfg(feature = "postgresql")]
    POSTGRES,
    #[cfg(feature = "mysql")]
    MYSQL,
    #[cfg(feature = "sqlite")]
    SQLITE,
];
