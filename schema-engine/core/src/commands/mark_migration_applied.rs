use crate::{CoreError, CoreResult};
use json_rpc::types::MigrationList;
use schema_connector::{
    SchemaConnector,
    migrations_directory::{MigrationDirectory, error_on_changed_provider},
};
use user_facing_errors::schema_engine::{MigrationAlreadyApplied, MigrationToMarkAppliedNotFound};

/// Mark a migration as applied in the migrations table.
///
/// There are two possible outcomes:
///
/// - The migration is already in the table, but in a failed state. In this case, we will mark it
///   as rolled back, then create a new entry.
/// - The migration is not in the table. We will create a new entry in the migrations table. The
///   `started_at` and `finished_at` will be the same.
/// - If it is already applied, we return a user-facing error.
#[derive(Debug)]
pub struct MarkMigrationAppliedInput {
    /// The name of the migration to mark applied.
    pub migration_name: String,

    /// The list of migrations, already loaded from disk.
    pub migrations_list: MigrationList,
}

/// The output of the `markMigrationApplied` command.
#[derive(Debug)]
pub struct MarkMigrationAppliedOutput {}

/// Mark a migration as applied.
pub async fn mark_migration_applied(
    input: MarkMigrationAppliedInput,
    connector: &mut dyn SchemaConnector,
) -> CoreResult<MarkMigrationAppliedOutput> {
    error_on_changed_provider(&input.migrations_list.lockfile, connector.connector_type())?;

    connector.acquire_lock().await?;

    let migration_directory = input
        .migrations_list
        .migration_directories
        .into_iter()
        .map(MigrationDirectory::new)
        .find(|dir| input.migration_name == dir.migration_name())
        .ok_or_else(|| {
            CoreError::user_facing(MigrationToMarkAppliedNotFound {
                migration_name: input.migration_name.clone(),
            })
        })?;

    let script = migration_directory.read_migration_script().map_err(|_err| {
        CoreError::user_facing(MigrationToMarkAppliedNotFound {
            migration_name: input.migration_name.clone(),
        })
    })?;

    let relevant_migrations = match connector.migration_persistence().list_migrations().await? {
        Ok(migrations) => migrations
            .into_iter()
            .filter(|migration| migration.migration_name == input.migration_name)
            .collect(),
        Err(_) => {
            connector.migration_persistence().baseline_initialize().await?;

            vec![]
        }
    };

    if relevant_migrations
        .iter()
        .any(|migration| migration.finished_at.is_some())
    {
        return Err(CoreError::user_facing(MigrationAlreadyApplied {
            migration_name: input.migration_name.clone(),
        }));
    }

    let migrations_to_mark_rolled_back = relevant_migrations
        .iter()
        .filter(|migration| migration.finished_at.is_none() && migration.rolled_back_at.is_none());

    for migration in migrations_to_mark_rolled_back {
        connector
            .migration_persistence()
            .mark_migration_rolled_back_by_id(&migration.id)
            .await?;
    }

    connector
        .migration_persistence()
        .mark_migration_applied(migration_directory.migration_name(), &script)
        .await?;

    Ok(MarkMigrationAppliedOutput {})
}
