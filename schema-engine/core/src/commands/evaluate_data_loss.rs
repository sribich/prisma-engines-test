use crate::{CoreResult, SchemaContainerExt, migration_schema_cache::MigrationSchemaCache};
use json_rpc::types::{MigrationList, SchemasContainer};
use psl::parser_database::ExtensionTypes;
use schema_connector::{SchemaConnector, migrations_directory::*};

/// Development command for migrations. Evaluate the data loss induced by the next
/// migration the engine would generate on the main database.
///
/// At this stage, the engine does not create or mutate anything in the database
/// nor in the migrations directory.
///
/// This is part of the `migrate dev` flow.
///
/// **Note**: the engine currently assumes the main database schema is up-to-date
/// with the migration history.
#[derive(Debug)]
pub struct EvaluateDataLossInput {
    /// The list of migrations, already loaded from disk.
    pub migrations_list: MigrationList,
    /// The prisma schema files to migrate to.
    pub schema: SchemasContainer,
}

/// The output of the `evaluateDataLoss` command.
#[derive(Debug)]
pub struct EvaluateDataLossOutput {
    /// The number migration steps that would be generated. If this is empty, we
    /// wouldn't generate a new migration, unless the `draft` option is
    /// passed.
    pub migration_steps: u32,
    /// Steps that cannot be executed on the local database in the
    /// migration that would be generated.
    pub unexecutable_steps: Vec<MigrationFeedback>,
    /// Destructive change warnings for the local database. These are the
    /// warnings *for the migration that would be generated*. This does not
    /// include other potentially yet unapplied migrations.
    pub warnings: Vec<MigrationFeedback>,
}

/// A data loss warning or an unexecutable migration error, associated with the step that triggered it.
#[derive(Debug)]
pub struct MigrationFeedback {
    /// The human-readable message.
    pub message: String,
    /// The index of the step this pertains to.
    pub step_index: u32,
}

/// Development command for migrations. Evaluate the data loss induced by the
/// next migration the engine would generate on the main database.
///
/// At this stage, the engine does not create or mutate anything in the database
/// nor in the migrations directory.
pub async fn evaluate_data_loss(
    input: EvaluateDataLossInput,
    connector: &mut dyn SchemaConnector,
    migration_schema_cache: &mut MigrationSchemaCache,
    extension_types: &dyn ExtensionTypes,
) -> CoreResult<EvaluateDataLossOutput> {
    error_on_changed_provider(&input.migrations_list.lockfile, connector.connector_type())?;
    let sources: Vec<_> = input.schema.to_psl_input();

    let migrations = Migrations::from_migration_list(&input.migrations_list);
    let dialect = connector.schema_dialect();

    let to = dialect.schema_from_datamodel(sources, connector.default_runtime_namespace(), extension_types)?;

    let from = migration_schema_cache
        .get_or_insert(&input.migrations_list.migration_directories, || async {
            // We only consider the namespaces present in the "to" schema aka the PSL file for the introspection of the "from" schema.
            // So when the user removes a previously existing namespace from their PSL file we will not introspect that namespace in the database.
            let namespaces = dialect.extract_namespaces(&to);
            connector.schema_from_migrations(&migrations, namespaces).await
        })
        .await?;

    let migration = dialect.diff(from, to);

    let migration_steps = dialect.migration_len(&migration) as u32;
    let diagnostics = connector.destructive_change_checker().check(&migration).await?;

    let warnings = diagnostics
        .warnings
        .into_iter()
        .map(|warning| MigrationFeedback {
            message: warning.description,
            step_index: warning.step_index as u32,
        })
        .collect();

    let unexecutable_steps = diagnostics
        .unexecutable_migrations
        .into_iter()
        .map(|unexecutable| MigrationFeedback {
            message: unexecutable.description,
            step_index: unexecutable.step_index as u32,
        })
        .collect();

    Ok(EvaluateDataLossOutput {
        migration_steps,
        warnings,
        unexecutable_steps,
    })
}
