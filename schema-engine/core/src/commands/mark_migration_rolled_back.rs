use crate::{CoreError, CoreResult};
use schema_connector::SchemaConnector;
use user_facing_errors::schema_engine::{CannotRollBackSucceededMigration, CannotRollBackUnappliedMigration};

/// Mark an existing failed migration as rolled back in the migrations table. It
/// will still be there, but ignored for all purposes except as audit trail.
#[derive(Debug)]
pub struct MarkMigrationRolledBackInput {
    /// The name of the migration to mark rolled back.
    pub migration_name: String,
}

/// The output of the `markMigrationRolledBack` command.
#[derive(Debug)]
pub struct MarkMigrationRolledBackOutput {}

/// Mark a migration as rolled back.
pub async fn mark_migration_rolled_back(
    input: MarkMigrationRolledBackInput,
    connector: &mut dyn SchemaConnector,
) -> CoreResult<MarkMigrationRolledBackOutput> {
    connector.acquire_lock().await?;

    let all_migrations = connector
        .migration_persistence()
        .list_migrations()
        .await?
        .map_err(|_err| {
            CoreError::from_msg(
                "Invariant violation: called markMigrationRolledBack on a database without migrations table.".into(),
            )
        })?;

    let relevant_migrations: Vec<_> = all_migrations
        .into_iter()
        .filter(|migration| migration.migration_name == input.migration_name)
        .collect();

    if relevant_migrations.is_empty() {
        return Err(CoreError::user_facing(CannotRollBackUnappliedMigration {
            migration_name: input.migration_name.clone(),
        }));
    }

    if relevant_migrations
        .iter()
        .all(|migration| migration.finished_at.is_some())
    {
        return Err(CoreError::user_facing(CannotRollBackSucceededMigration {
            migration_name: input.migration_name.clone(),
        }));
    }

    let migrations_to_roll_back = relevant_migrations
        .iter()
        .filter(|migration| migration.finished_at.is_none() && migration.rolled_back_at.is_none());

    for migration in migrations_to_roll_back {
        tracing::info!(
            migration_id = migration.id.as_str(),
            migration_name = migration.migration_name.as_str(),
            "Marking migration as rolled back."
        );
        connector
            .migration_persistence()
            .mark_migration_rolled_back_by_id(&migration.id)
            .await?;
    }

    Ok(MarkMigrationRolledBackOutput {})
}
