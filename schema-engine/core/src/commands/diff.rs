use std::sync::Arc;

use crate::{
    SchemaContainerExt, core_error::CoreResult, dialect_for_provider, extract_namespaces
};
use enumflags2::BitFlags;
use json_rpc::types::{MigrationList, SchemasContainer, SchemasWithConfigDir, UrlContainer};
use psl::parser_database::ExtensionTypes;
use schema_connector::{
    ConnectorError, ConnectorHost, DatabaseSchema, ExternalShadowDatabase, Namespaces, SchemaConnector, SchemaDialect,
    migrations_directory::Migrations,
};
use sql_schema_connector::SqlSchemaConnector;

/// The type of params for the `diff` method.
#[derive(Debug)]
pub struct DiffParams {
    /// The source of the schema to consider as a _starting point_.
    pub from: DiffTarget,

    /// The source of the schema to consider as a _destination_, or the desired
    /// end-state.
    pub to: DiffTarget,

    /// The URL to a live database to use as a shadow database. The schema and data on
    /// that database will be wiped during diffing.
    ///
    /// This is only necessary when one of `from` or `to` is referencing a migrations
    /// directory as a source for the schema.
    /// @deprecated.
    pub shadow_database_url: Option<String>,

    /// By default, the response will contain a human-readable diff. If you want an
    /// executable script, pass the `"script": true` param.
    pub script: bool,

    /// Whether the --exit-code param was passed.
    ///
    /// If this is set, the engine will return exitCode = 2 in the diffResult in case the diff is
    /// non-empty. Other than this, it does not change the behaviour of the command.
    pub exit_code: Option<bool>,
}

/// The result type for the `diff` method.
#[derive(Debug)]
pub struct DiffResult {
    /// The exit code that the CLI should return.
    pub exit_code: u32,

    /// The diff script, if `script` was set to true in [`DiffParams`](DiffParams),
    /// or a human-readable migration summary otherwise.
    /// This is meant to be printed to the stdout by the caller.
    /// Note: in `schema-engine-cli`, this is None.
    pub stdout: Option<String>,
}

/// A supported source for a database schema to diff in the `diff` command.
#[derive(Debug)]
pub enum DiffTarget {
    /// An empty schema.
    Empty,

    /// The Prisma schema content. The _datasource url_ will be considered, and the
    /// live database it points to introspected for its schema.
    SchemaDatasource(SchemasWithConfigDir),

    /// The Prisma schema content. The contents of the schema itself will be
    /// considered. This source does not need any database connection.
    SchemaDatamodel(SchemasContainer),

    /// The url to a live database. Its schema will be considered.
    ///
    /// This will cause the schema engine to connect to the database and read from it.
    /// It will not write.
    Url(UrlContainer),

    /// The Prisma schema content for migrations. The migrations will be applied to a **shadow database**, and the resulting schema
    /// considered for diffing.
    Migrations(MigrationList),
}

///
pub async fn diff(
    params: DiffParams,
    host: Arc<dyn ConnectorHost>,
    extension_types: &dyn ExtensionTypes,
) -> CoreResult<DiffResult> {
    // In order to properly handle MultiSchema, we need to make sure the preview feature is
    // correctly set, and we need to grab the namespaces from the Schema, if any.
    // Note that currently, we union all namespaces and preview features. This may not be correct.
    // TODO: This effectively reads and parses (parts of) the schema twice: once here, and once
    // below, when defining 'from'/'to'. We should revisit this.
    let (namespaces, preview_features) =
        namespaces_and_preview_features_from_diff_targets(&[&params.from, &params.to])?;

    let from = json_rpc_diff_target_to_dialect(
        &params.from,
        params.shadow_database_url.as_deref(),
        namespaces.clone(),
        preview_features,
        extension_types,
    )
    .await?;
    let to = json_rpc_diff_target_to_dialect(
        &params.to,
        params.shadow_database_url.as_deref(),
        namespaces,
        preview_features,
        extension_types,
    )
    .await?;

    // The `from` connector takes precedence, because if we think of diffs as migrations, `from` is
    // the target where the migration would be applied.
    //
    // TODO: make sure the shadow_database_url param is _always_ taken into account.
    // TODO: make sure the connectors are the same in from and to.
    let (dialect, from, to) = match (from, to) {
        (Some((connector, from)), Some((_, to))) => (connector, from, to),
        (Some((connector, from)), None) => {
            let to = connector.empty_database_schema();
            (connector, from, to)
        }
        (None, Some((connector, to))) => {
            let from = connector.empty_database_schema();
            (connector, from, to)
        }
        (None, None) => {
            return Err(ConnectorError::from_msg(
                "Could not determine the connector to use for diffing.".to_owned(),
            ));
        }
    };

    let migration = dialect.diff(from, to);

    let mut stdout = if params.script {
        dialect.render_script(&migration, &Default::default())?
    } else {
        dialect.migration_summary(&migration)
    };

    if !stdout.ends_with('\n') {
        stdout.push('\n');
    }

    host.print(&stdout).await?;

    let exit_code = if params.exit_code == Some(true) && !dialect.migration_is_empty(&migration) {
        2
    } else {
        0
    };

    Ok(DiffResult {
        exit_code,
        stdout: None,
    })
}

// Grab the preview features and namespaces. Normally, we can only grab these from Schema files,
// and we usually only expect one of these within a set of DiffTarget.
// However, in case there's multiple, we union the results. This may be wrong.
fn namespaces_and_preview_features_from_diff_targets(
    targets: &[&DiffTarget],
) -> CoreResult<(Option<Namespaces>, BitFlags<psl::PreviewFeature>)> {
    let mut namespaces = Vec::new();
    let mut preview_features = BitFlags::default();

    for target in targets {
        match target {
            DiffTarget::Migrations(_) | DiffTarget::Empty | DiffTarget::Url(_) => (),
            DiffTarget::SchemaDatasource(schemas) => {
                let sources = (&schemas.files).to_psl_input();

                extract_namespaces(&sources, &mut namespaces, &mut preview_features);
            }
            DiffTarget::SchemaDatamodel(schemas) => {
                let sources = (&schemas.files).to_psl_input();

                extract_namespaces(&sources, &mut namespaces, &mut preview_features);
            }
        }
    }

    Ok((Namespaces::from_vec(&mut namespaces), preview_features))
}

// `None` in case the target is empty
async fn json_rpc_diff_target_to_dialect(
    target: &DiffTarget,
    shadow_database_url: Option<&str>, // TODO: delete the parameter
    namespaces: Option<Namespaces>,
    preview_features: BitFlags<psl::PreviewFeature>,
    extension_types: &dyn ExtensionTypes,
) -> CoreResult<Option<(Box<dyn SchemaDialect>, DatabaseSchema)>> {
    match target {
        DiffTarget::Empty => Ok(None),
        DiffTarget::SchemaDatasource(schemas) => {
            let config_dir = std::path::Path::new(&schemas.config_dir);
            let sources: Vec<_> = schemas.to_psl_input();

            // actually, just use the given `connector`. Verify that the provider is the same
            // as the one assumed by the connector.

            let mut connector = crate::schema_to_connector(&sources, Some(config_dir))?;
            connector.ensure_connection_validity().await?;
            connector.set_preview_features(preview_features);

            let schema = connector.schema_from_database(namespaces).await?;
            Ok(Some((connector.schema_dialect(), schema)))
        }
        DiffTarget::SchemaDatamodel(schemas) => {
            let sources = schemas.to_psl_input();

            // Connector only needed to infer the default namespace.
            // If connector cannot be created (e.g. due to invalid or missing URL) we use the dialect's default namespace.
            let (default_namespace, dialect) = match crate::schema_to_connector(&sources, None) {
                Ok(connector) => (
                    connector.default_runtime_namespace().map(|ns| ns.to_string()),
                    connector.schema_dialect(),
                ),
                Err(_) => {
                    let dialect = crate::schema_to_dialect(&sources)?;
                    (dialect.default_namespace().map(|ns| ns.to_string()), dialect)
                }
            };


            let schema = dialect.schema_from_datamodel(sources, default_namespace.as_deref(), extension_types)?;

            Ok(Some((dialect, schema)))
        }
        DiffTarget::Url(UrlContainer { url }) => {
            // this will not be supported

            let mut connector = crate::connector_for_connection_string(url.clone(), None, BitFlags::empty())?;
            connector.ensure_connection_validity().await?;
            connector.set_preview_features(preview_features);

            let schema = connector.schema_from_database(namespaces).await?;
            let dialect = connector.schema_dialect();

            connector.dispose().await?;

            Ok(Some((dialect, schema)))
        }
        DiffTarget::Migrations(migration_list) => {
            let provider =
                schema_connector::migrations_directory::read_provider_from_lock_file(&migration_list.lockfile);
            match (provider.as_deref(), shadow_database_url) {
                (Some(provider), Some(shadow_database_url)) => {
                    let dialect = dialect_for_provider(provider)?;
                    let migrations = Migrations::from_migration_list(migration_list);


                    let schema = dialect
                        .schema_from_migrations_with_target(
                            &migrations,
                            namespaces,
                            ExternalShadowDatabase::ConnectionString {
                                connection_string: shadow_database_url.to_owned(),
                                preview_features,
                            },
                        )
                        .await?;
                    Ok(Some((dialect, schema)))
                }
                (Some("sqlite"), None) => {
                    // TODO: we don't need this branch
                    let mut connector = SqlSchemaConnector::new_sqlite_inmem(preview_features)?;
                    let migrations = Migrations::from_migration_list(migration_list);

                    let schema = connector
                        .schema_from_migrations(&migrations, namespaces)
                        .await?;
                    Ok(Some((connector.schema_dialect(), schema)))
                }
                (Some(_), None) => Err(ConnectorError::from_msg(
                    "You must pass the --shadow-database-url if you want to diff a migrations directory.".to_owned(),
                )),
                (None, _) => Err(ConnectorError::from_msg(
                    "Could not determine the connector from the migrations directory (missing migration_lock.toml)."
                        .to_owned(),
                )),
            }
        }
    }
}
