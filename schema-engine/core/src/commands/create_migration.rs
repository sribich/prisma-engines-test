use crate::{CoreError, CoreResult, SchemaContainerExt, migration_schema_cache::MigrationSchemaCache};
use crosstarget_utils::time::format_utc_now;
use json_rpc::types::{MigrationList, SchemasContainer};
use psl::parser_database::ExtensionTypes;
use schema_connector::{SchemaConnector, migrations_directory::*};
use user_facing_errors::schema_engine::MigrationNameTooLong;

/// The input to the `createMigration` command.
#[derive(Debug)]
pub struct CreateMigrationInput {
    /// If true, always generate a migration, but do not apply.
    pub draft: bool,

    /// The user-given name for the migration. This will be used for the migration directory.
    pub migration_name: String,

    /// The list of migrations, already loaded from disk.
    pub migrations_list: MigrationList,

    /// The Prisma schema content to use as a target for the generated migration.
    pub schema: SchemasContainer,
}

/// The output of the `createMigration` command.
#[derive(Debug)]
pub struct CreateMigrationOutput {
    /// The active connector type used.
    pub connector_type: String,

    /// The generated name of migration directory, which the caller must use to create the new directory.
    pub generated_migration_name: String,

    /// The migration script that was generated, if any.
    /// It will be null if:
    /// 1. The migration we generate would be empty, **AND**
    /// 2. the `draft` param was not true, because in that case the engine would still generate an empty
    ///    migration script.
    pub migration_script: Option<String>,

    /// The file extension for generated migration files.
    pub extension: String,
}

/// Create a directory name for a new migration.
pub fn generate_migration_directory_name(migration_name: &str) -> String {
    let timestamp = format_utc_now("%Y%m%d%H%M%S");
    if migration_name.is_empty() {
        timestamp
    } else {
        format!("{timestamp}_{migration_name}")
    }
}

/// Create a new migration.
pub async fn create_migration(
    input: CreateMigrationInput,
    connector: &mut dyn SchemaConnector,
    migration_schema_cache: &mut MigrationSchemaCache,
    extension_types: &dyn ExtensionTypes,
) -> CoreResult<CreateMigrationOutput> {
    let connector_type = connector.connector_type();

    if input.migration_name.len() > 200 {
        return Err(CoreError::user_facing(MigrationNameTooLong));
    }

    // Check for provider switch
    error_on_changed_provider(&input.migrations_list.lockfile, connector_type)?;

    let generated_migration_name = generate_migration_directory_name(&input.migration_name);

    // Infer the migration.
    let migrations = Migrations::from_migration_list(&input.migrations_list);
    let sources: Vec<_> = input.schema.to_psl_input();
    let dialect = connector.schema_dialect();

    let default_namespace = connector.default_runtime_namespace();
    // We need to start with the 'to', which is the Schema, in order to grab the
    // namespaces, in case we've got MultiSchema enabled.
    let to = dialect.schema_from_datamodel(sources, default_namespace, extension_types)?;
    let namespaces = dialect.extract_namespaces(&to);

    let from = migration_schema_cache
        .get_or_insert(&input.migrations_list.migration_directories, || async {
            // We pass the namespaces here, because we want to describe all of the namespaces we know about from the "to" schema.
            connector.schema_from_migrations(&migrations, namespaces).await
        })
        .await?;

    let migration = dialect.diff(from, to);

    let extension = dialect.migration_file_extension().to_owned();

    if dialect.migration_is_empty(&migration) && !input.draft {
        tracing::info!("Database is up-to-date, returning without creating new migration.");

        return Ok(CreateMigrationOutput {
            connector_type: connector_type.to_owned(),
            generated_migration_name,
            migration_script: None,
            extension,
        });
    }

    let destructive_change_diagnostics = connector.destructive_change_checker().pure_check(&migration);

    let migration_script = dialect.render_script(&migration, &destructive_change_diagnostics)?;

    Ok(CreateMigrationOutput {
        connector_type: connector_type.to_owned(),
        generated_migration_name,
        migration_script: Some(migration_script),
        extension,
    })
}
