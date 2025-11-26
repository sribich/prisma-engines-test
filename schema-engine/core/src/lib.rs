#![deny(rust_2018_idioms, unsafe_code, /*missing_docs*/)]
#![allow(clippy::needless_collect)] // the implementation of that rule is way too eager, it rejects necessary collects
#![allow(clippy::derive_partial_eq_without_eq)]

//! The top-level library crate for the schema engine.

// exposed for tests
pub mod commands;

pub use core_error::{CoreError, CoreResult};
pub use json_rpc;

mod core_error;
mod extensions;
mod migration_schema_cache;
pub mod state;
mod timings;

use crate::state::EngineState;

pub use self::timings::TimingsLayer;
pub use extensions::{ExtensionType, ExtensionTypeConfig};
use json_rpc::types::{SchemaContainer, SchemasContainer, SchemasWithConfigDir};
pub use schema_connector;

use enumflags2::BitFlags;
use psl::{
    Datasource, PreviewFeature, ValidatedSchema, builtin_connectors::*, datamodel_connector::Flavour, parser_database::{ExtensionTypes, SourceFile}
};
use schema_connector::ConnectorParams;
use sql_schema_connector::{SqlSchemaConnector, SqlSchemaDialect};
use std::{env, path::Path, sync::Arc};
use user_facing_errors::common::InvalidConnectionString;


/// Creates the [`SqlSchemaDialect`](SqlSchemaDialect) matching the given provider.
pub fn dialect_for_provider(provider: &str) -> CoreResult<Box<dyn schema_connector::SchemaDialect>> {
    let error = || {
        Err(CoreError::from_msg(format!(
            "`{provider}` is not a supported connector."
        )))
    };

    if let Some(connector) = BUILTIN_CONNECTORS.iter().find(|c| c.is_provider(provider)) {
        match connector.flavour() {
            Flavour::Sqlite => Ok(Box::new(SqlSchemaDialect::sqlite())),
            Flavour::Postgres => Ok(Box::new(SqlSchemaDialect::postgres())),
            Flavour::Mysql => Ok(Box::new(SqlSchemaDialect::mysql())),
            #[allow(unreachable_patterns)]
            _ => error(),
        }
    } else {
        error()
    }
}

/// Extracts the database namespaces from the given schema files.
pub fn extract_namespaces(
    files: &[(String, psl::SourceFile)],
    namespaces: &mut Vec<String>,
    preview_features: &mut BitFlags<psl::PreviewFeature>,
) {
    let validated_schema = psl::validate_multi_file_without_extensions(files);

    for (namespace, _span) in validated_schema
        .configuration
        .datasources
        .iter()
        .flat_map(|ds| ds.namespaces.iter())
    {
        namespaces.push(namespace.clone());
    }

    for generator in &validated_schema.configuration.generators {
        *preview_features |= generator.preview_features.unwrap_or_default();
    }
}

fn connector_for_connection_string(
    connection_string: String,
    shadow_database_connection_string: Option<String>,
    preview_features: BitFlags<PreviewFeature>,
) -> CoreResult<Box<dyn schema_connector::SchemaConnector>> {
    match connection_string.split(':').next() {
        Some("postgres") | Some("postgresql") | Some("prisma+postgres") => {
            let params = ConnectorParams {
                connection_string,
                preview_features,
                shadow_database_connection_string,
            };
            Ok(Box::new(SqlSchemaConnector::new_postgres_like(params)?))
        }
        Some("file") => {
            let params = ConnectorParams {
                connection_string,
                preview_features,
                shadow_database_connection_string,
            };
            Ok(Box::new(SqlSchemaConnector::new_sqlite(params)?))
        }
        Some("mysql") => {
            let params = ConnectorParams {
                connection_string,
                preview_features,
                shadow_database_connection_string,
            };
            Ok(Box::new(SqlSchemaConnector::new_mysql(params)?))
        }
        Some(_other) => Err(CoreError::url_parse_error("The scheme is not recognized")),
        None => Err(CoreError::user_facing(InvalidConnectionString {
            details: String::new(),
        })),
    }
}

/// Same as schema_to_connector, but it will only read the provider, not the connector params.
/// This uses `schema_files` to read `preview_features` and the `datasource` block.
fn schema_to_dialect(schema_files: &[(String, SourceFile)]) -> CoreResult<Box<dyn schema_connector::SchemaDialect>> {
    let (_, config) = psl::parse_configuration_multi_file(schema_files)
        .map_err(|(files, err)| CoreError::new_schema_parser_error(files.render_diagnostics(&err)))?;

    let preview_features = config.preview_features();
    let datasource = config
        .datasources
        .into_iter()
        .next()
        .ok_or_else(|| CoreError::from_msg("There is no datasource in the schema.".into()))?;

    if let Ok(connection_string) = datasource.load_direct_url(|key| env::var(key).ok()) {
        // TODO: remove conditional branch in Prisma 7.
        let connector_params = ConnectorParams {
            connection_string,
            preview_features,
            shadow_database_connection_string: datasource.load_shadow_database_url().ok().flatten(),
        };
        let conn = connector_for_provider(datasource.active_provider, connector_params)?;
        Ok(conn.schema_dialect())
    } else {
        dialect_for_provider(datasource.active_provider)
    }
}

/// Go from a schema to a connector.
fn schema_to_connector(
    files: &[(String, SourceFile)],
    config_dir: Option<&Path>,
) -> CoreResult<Box<dyn schema_connector::SchemaConnector>> {
    let (datasource, url, preview_features, shadow_database_url) = parse_configuration_multi(files)?;

    let url = config_dir
        .map(|config_dir| psl::set_config_dir(datasource.active_connector.flavour(), config_dir, &url).into_owned())
        .unwrap_or(url);

    let params = ConnectorParams {
        connection_string: url,
        preview_features,
        shadow_database_connection_string: shadow_database_url,
    };

    connector_for_provider(datasource.active_provider, params)
}

fn initial_datamodel_to_connector(
    initial_datamodel: &psl::ValidatedSchema,
) -> CoreResult<Box<dyn schema_connector::SchemaConnector>> {
    let configuration = &initial_datamodel.configuration;
    let (datasource, url, preview_features, shadow_database_url) = extract_configuration_ref(configuration, |_| {
        CoreError::new_schema_parser_error(initial_datamodel.render_own_diagnostics())
    })?;

    let params = ConnectorParams {
        connection_string: url,
        preview_features,
        shadow_database_connection_string: shadow_database_url,
    };

    connector_for_provider(datasource.active_provider, params)
}

fn connector_for_provider(
    provider: &str,
    params: ConnectorParams,
) -> CoreResult<Box<dyn schema_connector::SchemaConnector>> {
    if let Some(connector) = BUILTIN_CONNECTORS.iter().find(|c| c.is_provider(provider)) {
        match connector.flavour() {
            Flavour::Mysql => Ok(Box::new(SqlSchemaConnector::new_mysql(params)?)),
            Flavour::Postgres => Ok(Box::new(SqlSchemaConnector::new_postgres(params)?)),
            Flavour::Sqlite => Ok(Box::new(SqlSchemaConnector::new_sqlite(params)?)),
        }
    } else {
        Err(CoreError::from_msg(format!(
            "`{provider}` is not a supported connector."
        )))
    }
}

/// Top-level constructor for the schema engine API.
/// This variant does not support extensions.
pub fn schema_api_without_extensions(
    datamodel: Option<String>,
    host: Option<std::sync::Arc<dyn schema_connector::ConnectorHost>>,
) -> CoreResult<Box<EngineState>> {
    schema_api(datamodel, host, Arc::new(ExtensionTypeConfig::default()))
}

/// Top-level constructor for the schema engine API.
pub fn schema_api(
    datamodel: Option<String>,
    host: Option<std::sync::Arc<dyn schema_connector::ConnectorHost>>,
    extension_config: Arc<ExtensionTypeConfig>,
) -> CoreResult<Box<EngineState>> {
    // Eagerly load the default schema, for validation errors.
    if let Some(datamodel) = &datamodel {
        parse_configuration(datamodel)?;
    }

    let datamodel = datamodel.map(|datamodel| vec![("schema.prisma".to_owned(), SourceFile::from(datamodel))]);

    let state = state::EngineState::new(datamodel, host, extension_config);
    Ok(Box::new(state))
}

fn parse_configuration(datamodel: &str) -> CoreResult<(Datasource, String, BitFlags<PreviewFeature>, Option<String>)> {
    let config = psl::parse_configuration(datamodel)
        .map_err(|err| CoreError::new_schema_parser_error(err.to_pretty_string("schema.prisma", datamodel)))?;

    extract_configuration(config, |err| {
        CoreError::new_schema_parser_error(err.to_pretty_string("schema.prisma", datamodel))
    })
}

fn parse_configuration_multi(
    files: &[(String, SourceFile)],
) -> CoreResult<(Datasource, String, BitFlags<PreviewFeature>, Option<String>)> {
    let (files, mut config) = psl::parse_configuration_multi_file(files)
        .map_err(|(files, err)| CoreError::new_schema_parser_error(files.render_diagnostics(&err)))?;

    extract_configuration(config, |err| {
        CoreError::new_schema_parser_error(files.render_diagnostics(&err))
    })
}

fn parse_schema_multi(
    files: &[(String, SourceFile)],
    extension_types: &dyn ExtensionTypes,
) -> CoreResult<ValidatedSchema> {
    psl::parse_schema_multi(files, extension_types).map_err(CoreError::new_schema_parser_error)
}

fn extract_configuration(
    config: psl::Configuration,
    mut err_handler: impl FnMut(psl::Diagnostics) -> CoreError,
) -> CoreResult<(Datasource, String, BitFlags<PreviewFeature>, Option<String>)> {
    let preview_features = config.preview_features();

    let source = config
        .datasources
        .into_iter()
        .next()
        .ok_or_else(|| CoreError::from_msg("There is no datasource in the schema.".into()))?;

    let url = source
        .load_direct_url(|key| env::var(key).ok())
        .map_err(&mut err_handler)?;

    let shadow_database_url = source.load_shadow_database_url().map_err(err_handler)?;

    Ok((source, url, preview_features, shadow_database_url))
}

fn extract_configuration_ref(
    config: &psl::Configuration,
    mut err_handler: impl Fn(psl::Diagnostics) -> CoreError,
) -> CoreResult<(&Datasource, String, BitFlags<PreviewFeature>, Option<String>)> {
    let preview_features = config.preview_features();

    let source = config
        .datasources
        .first()
        .ok_or_else(|| CoreError::from_msg("There is no datasource in the schema.".into()))?;

    let url = source
        .load_direct_url(|key| env::var(key).ok())
        .map_err(&mut err_handler)?;

    let shadow_database_url = source.load_shadow_database_url().map_err(err_handler)?;

    Ok((source, url, preview_features, shadow_database_url))
}

trait SchemaContainerExt {
    fn to_psl_input(self) -> Vec<(String, SourceFile)>;
}

impl SchemaContainerExt for SchemasContainer {
    fn to_psl_input(self) -> Vec<(String, SourceFile)> {
        self.files.to_psl_input()
    }
}

impl SchemaContainerExt for &SchemasContainer {
    fn to_psl_input(self) -> Vec<(String, SourceFile)> {
        (&self.files).to_psl_input()
    }
}

impl SchemaContainerExt for Vec<SchemaContainer> {
    fn to_psl_input(self) -> Vec<(String, SourceFile)> {
        self.into_iter()
            .map(|container| (container.path, SourceFile::from(container.content)))
            .collect()
    }
}

impl SchemaContainerExt for Vec<&SchemaContainer> {
    fn to_psl_input(self) -> Vec<(String, SourceFile)> {
        self.into_iter()
            .map(|container| (container.path.clone(), SourceFile::from(&container.content)))
            .collect()
    }
}

impl SchemaContainerExt for &Vec<SchemaContainer> {
    fn to_psl_input(self) -> Vec<(String, SourceFile)> {
        self.iter()
            .map(|container| (container.path.clone(), SourceFile::from(&container.content)))
            .collect()
    }
}

impl SchemaContainerExt for &SchemasWithConfigDir {
    fn to_psl_input(self) -> Vec<(String, SourceFile)> {
        (&self.files).to_psl_input()
    }
}
