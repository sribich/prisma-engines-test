use std::sync::Arc;

use anyhow::Result;
use clap::{Parser, Subcommand};
use psl::{ConfigOnlyParser, Datasource};
use schema_ast::SourceFile;
use schema_context::{SchemaContext, load_schema_context};
use schema_core::{
    ExtensionTypeConfig,
    commands::{create_database::CreateDatabaseParams, dev_diagnostic::DevDiagnosticInput, diagnose_migration_history::DiagnoseMigrationHistoryInput, ensure_connection_validity::EnsureConnectionValidityParams},
    json_rpc::types::{
        DatasourceParam,
        MigrationList, MigrationLockfile, UrlContainer,
    },
    state::EngineState,
};
use tracing::debug;

use crate::path::diff_paths;

#[derive(Parser)]
#[command(name = "workspace")]
pub struct MigrateCli {
    #[command(subcommand)]
    command: MigrateCommand,
}

#[derive(Subcommand)]
pub enum MigrateCommand {
    #[command(about = "")]
    Deploy,
    #[command(about = "")]
    Dev,
    #[command(about = "")]
    Diff,
    #[command(about = "")]
    Reset,
    #[command(about = "")]
    Resolve,
    #[command(about = "")]
    Status,
}

#[derive(Parser)]
pub struct DeployArgs {}

pub async fn run(args: MigrateCli) -> Result<()> {
    match args.command {
        MigrateCommand::Deploy => migrate_deploy().await,
        MigrateCommand::Dev => migrate_dev().await,
        MigrateCommand::Diff => migrate_diff().await,
        MigrateCommand::Reset => migrate_reset().await,
        MigrateCommand::Resolve => migrate_resolve().await,
        MigrateCommand::Status => migrate_status().await,
    }
}

async fn migrate_deploy() -> Result<()> {
    // state.apply_migrations(input)
    Ok(())
}

async fn migrate_dev() -> Result<()> {
    let context = SchemaContext::load(None).unwrap();
    let config_context = context.parse::<ConfigOnlyParser>();

    let context = load_schema_context(None, None)?;

    let files = context
        .schemas
        .iter()
        .map(|path| {
            let relative_path = diff_paths(path, &context.root_dir).unwrap();
            let contents = std::fs::read_to_string(path).unwrap();

            (relative_path.to_str().unwrap().to_string(), contents.into())
        })
        .collect::<Vec<(String, SourceFile)>>();

    let mut state = EngineState::new(Some(files), None, Arc::new(ExtensionTypeConfig::default()));

    let url = config_context.context.inner.configuration.datasources[0]
        .get_connection_url()
        .clone()
        .value
        .unwrap()
        .to_owned();

    ensure_db_exists(&mut state, url).await?;

    let diagnostics = state
        .dev_diagnostic(DevDiagnosticInput {
            migrations_list: MigrationList {
                base_dir: context.root_dir.join("migrations").to_str().unwrap().to_string(),
                lockfile: MigrationLockfile {
                    path: context.root_dir.join("migrations.lock").to_str().unwrap().to_string(),
                    content: None,
                },
                shadow_db_init_script: "".to_owned(),
                migration_directories: vec![],
            },
        })
        .await;

    println!("{:#?}", diagnostics);

    Ok(())
}

async fn migrate_diff() -> Result<()> {
    // state.diff(params)
    Ok(())
}

async fn migrate_reset() -> Result<()> {
    // state.reset(input)
    Ok(())
}

async fn migrate_resolve() -> Result<()> {
    // state.mark_migration_applied(input)
    // state.rolled_back(input)

    Ok(())
}

async fn ensure_db_exists(state: &mut EngineState, url: String) -> Result<()> {
    let result = state
        .ensure_connection_validity(EnsureConnectionValidityParams {
            datasource: DatasourceParam::ConnectionString(UrlContainer { url: url.clone() }),
        })
        .await;

    if result.is_ok() {
        return Ok(());
    }

    if result.as_ref().unwrap_err().error_code() != Some("P1003") {
        panic!("Error {:#?}", result);
        // return Err(result.unwrap_err());
    }

    let result = state
        .create_database(CreateDatabaseParams {
            datasource: DatasourceParam::ConnectionString(UrlContainer { url }),
        })
        .await;

    Ok(())
}

async fn can_connect(state: &EngineState, url: String) -> bool {
    let result = state
        .ensure_connection_validity(EnsureConnectionValidityParams {
            datasource: DatasourceParam::ConnectionString(UrlContainer { url }),
        })
        .await;

    match result {
        Ok(_) => true,
        Err(error) => {
            let code = error.error_code();
            let message = error.message();

            if let (Some(code), Some(message)) = (code, message) {
                println!("{}: {}", code, message);
            } else {
                println!("Schema engine error: {}", error.to_user_facing().message());
            }

            false
        }
    }
}

async fn migrate_status() -> Result<()> {
    let context = SchemaContext::load(None).unwrap();
    let config_context = context.parse::<ConfigOnlyParser>();

    let context = load_schema_context(None, None)?;

    let files = context
        .schemas
        .iter()
        .map(|path| {
            let relative_path = diff_paths(path, &context.root_dir).unwrap();
            let contents = std::fs::read_to_string(path).unwrap();

            (relative_path.to_str().unwrap().to_string(), contents.into())
        })
        .collect::<Vec<(String, SourceFile)>>();

    println!("{:#?}", files);
    let state = EngineState::new(Some(files), None, Arc::new(ExtensionTypeConfig::default()));

    if !can_connect(
        &state,
        config_context.context.inner.configuration.datasources[0]
            .get_connection_url()
            .clone()
            .value
            .unwrap()
            .to_owned(),
    )
    .await
    {
        return Ok(());
    }

    println!("{:#?}", context);

    let history = state
        .diagnose_migration_history(DiagnoseMigrationHistoryInput {
            migrations_list: MigrationList {
                base_dir: context.root_dir.join("migrations").to_str().unwrap().to_string(),
                lockfile: MigrationLockfile {
                    path: context.root_dir.join("migrations.lock").to_str().unwrap().to_string(),
                    content: None,
                },
                shadow_db_init_script: "".to_owned(),
                migration_directories: vec![],
            },
            opt_in_to_shadow_database: false,
        })
        .await;

    debug!("{:#?}", history);

    let migrations = config_context.list_migrations();
    println!("{:#?}", migrations);

    Ok(())
}

/*

    await loadEnvFile({ schemaPath: args['--schema'], printMessage: true, config })

    const schemaContext = await loadSchemaContext({
      schemaPathFromArg: args['--schema'],
      schemaPathFromConfig: config.schema,
      schemaEngineConfig: config,
    })
    const { migrationsDirPath } = inferDirectoryConfig(schemaContext, config)

    printDatasource({ datasourceInfo: parseDatasourceInfo(schemaContext.primaryDatasource), adapter })

    const schemaFilter: MigrateTypes.SchemaFilter = {
      externalTables: config.tables?.external ?? [],
      externalEnums: config.enums?.external ?? [],
    }

    const migrate = await Migrate.setup({
      schemaEngineConfig: config,
      migrationsDirPath,
      schemaContext,
      schemaFilter,
      extensions: config['extensions'],
    })

    await ensureCanConnectToDatabase(schemaContext.primaryDatasource)

    // This is a *read-only* command (modulo shadow database).
    // - ↩️ **RPC**: ****`diagnoseMigrationHistory`, then four cases based on the response.
    //     4. Otherwise, there is no problem migrate is aware of. We could still display:
    //         - Modified since applied only relevant when using dev, they are ignored for deploy
    //         - Pending migrations (those in the migrations folder that haven't been applied yet)
    //         - If there are no pending migrations, tell the user everything looks OK and up to date.

    let diagnoseResult: EngineResults.DiagnoseMigrationHistoryOutput
    let listMigrationDirectoriesResult: EngineResults.ListMigrationDirectoriesOutput

    try {
      diagnoseResult = await migrate.diagnoseMigrationHistory({
        optInToShadowDatabase: false,
      })
      debug({ diagnoseResult: JSON.stringify(diagnoseResult, null, 2) })

      listMigrationDirectoriesResult = await migrate.listMigrationDirectories()
      debug({ listMigrationDirectoriesResult })
    } finally {
      await migrate.stop()
    }

    process.stdout.write('\n') // empty line

    if (listMigrationDirectoriesResult.migrations.length > 0) {
      const migrations = listMigrationDirectoriesResult.migrations
      process.stdout.write(
        `${migrations.length} migration${migrations.length > 1 ? 's' : ''} found in prisma/migrations\n`,
      )
    } else {
      process.stdout.write(`No migration found in prisma/migrations\n`)
    }

    let unappliedMigrations: string[] = []
    if (diagnoseResult.history?.diagnostic === 'databaseIsBehind') {
      unappliedMigrations = diagnoseResult.history.unappliedMigrationNames
      process.stdout.write(
        `Following migration${unappliedMigrations.length > 1 ? 's' : ''} have not yet been applied:
${unappliedMigrations.join('\n')}

To apply migrations in development run ${bold(green(getCommandWithExecutor(`prisma migrate dev`)))}.
To apply migrations in production run ${bold(green(getCommandWithExecutor(`prisma migrate deploy`)))}.\n`,
      )
      // Exit 1 to signal that the status is not in sync
      process.exit(1)
    } else if (diagnoseResult.history?.diagnostic === 'historiesDiverge') {
      console.error(`Your local migration history and the migrations table from your database are different:

The last common migration is: ${diagnoseResult.history.lastCommonMigrationName}

The migration${diagnoseResult.history.unappliedMigrationNames.length > 1 ? 's' : ''} have not yet been applied:
${diagnoseResult.history.unappliedMigrationNames.join('\n')}

The migration${
        diagnoseResult.history.unpersistedMigrationNames.length > 1 ? 's' : ''
      } from the database are not found locally in prisma/migrations:
${diagnoseResult.history.unpersistedMigrationNames.join('\n')}`)
      // Exit 1 to signal that the status is not in sync
      process.exit(1)
    }

    if (!diagnoseResult.hasMigrationsTable) {
      //         - This is the **baselining** case.
      //         - Look at the migrations in the migrations folder
      //             - There is no local migration
      //                 - ...and there is drift: the user is coming from db push or another migration tool.
      //                 - Guide the user to an init flow with introspect + SQL schema dump (optionally)
      //             - There are local migrations
      //                 - ↩️ **RPC** `listMigrationDirectories` ****Take the first (=oldest) migration.
      //                 - Suggest calling `prisma migrate resolve --applied <migration-name>`

      if (listMigrationDirectoriesResult.migrations.length === 0) {
        console.error(`The current database is not managed by Prisma Migrate.

Read more about how to baseline an existing production database:
${link('https://pris.ly/d/migrate-baseline')}`)
        // Exit 1 to signal that the status is not in sync
        process.exit(1)
      } else {
        const migrationId = listMigrationDirectoriesResult.migrations.shift() as string
        console.error(`The current database is not managed by Prisma Migrate.

If you want to keep the current database structure and data and create new migrations, baseline this database with the migration "${migrationId}":
${bold(green(getCommandWithExecutor(`prisma migrate resolve --applied "${migrationId}"`)))}

Read more about how to baseline an existing production database:
https://pris.ly/d/migrate-baseline`)
        // Exit 1 to signal that the status is not in sync
        process.exit(1)
      }
    } else if (diagnoseResult.failedMigrationNames.length > 0) {
      //         - This is the **recovering from a partially failed migration** case.
      //         - Inform the user that they can "close the case" and mark the failed migration as fixed by calling `prisma migrate resolve`.
      //             - `prisma migrate resolve --rolled-back <migration-name>` if the migration was rolled back
      //             - `prisma migrate resolve --applied <migration-name>` if the migration was rolled forward (and completed successfully)
      const failedMigrations = diagnoseResult.failedMigrationNames

      console.error(
        `Following migration${failedMigrations.length > 1 ? 's' : ''} have failed:
${failedMigrations.join('\n')}

During development if the failed migration(s) have not been deployed to a production database you can then fix the migration(s) and run ${bold(
          green(getCommandWithExecutor(`prisma migrate dev`)),
        )}.\n`,
      )

      console.error(`The failed migration(s) can be marked as rolled back or applied:

- If you rolled back the migration(s) manually:
${bold(green(getCommandWithExecutor(`prisma migrate resolve --rolled-back "${failedMigrations[0]}"`)))}

- If you fixed the database manually (hotfix):
${bold(green(getCommandWithExecutor(`prisma migrate resolve --applied "${failedMigrations[0]}"`)))}

Read more about how to resolve migration issues in a production database:
${link('https://pris.ly/d/migrate-resolve')}`)

      // Exit 1 to signal that the status is not in sync
      process.exit(1)
    } else {
      process.stdout.write('\n') // empty line
      if (unappliedMigrations.length === 0) {
        // Exit 0 to signal that the status is in sync
        return `Database schema is up to date!`
      }
    }

    // Only needed for the return type to match
    return ''
  }
*/
