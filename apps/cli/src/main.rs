mod command;
mod logger;
mod path;
mod url;
mod util;

use command::{debug::DebugCommand, migrate::MigrateCli};

use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use schema_context::SchemaContext;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
    #[command(flatten)]
    verbose: Verbosity,
}

#[derive(Subcommand)]
enum Command {
    Migrate(MigrateCli),
    Db,
    Generate,
    Version,
    Validate,
    Format,
    Telemetry,
    Debug(DebugCommand),
}

#[tokio::main]
async fn main() -> Result<()> {
    logger::init_logger();
    // tracing::info!(git_hash = env!("GIT_HASH"), "Starting schema engine RPC server",);

    let context = SchemaContext::load(None).unwrap();
    load_envs(&context).unwrap();

    let args = Cli::parse();

    match args.command {
        Command::Debug(debug_command) => command::debug::run(debug_command),
        Command::Migrate(migrate_command) => command::migrate::run(migrate_command).await,
        _ => todo!(),
    }?;

    Ok(())
}

fn load_envs(context: &SchemaContext) -> Result<()> {
    let env_path = context.root_dir.join(".env");

    if !env_path.is_file() {
        return Ok(());
    }

    for env in dotenv::from_path_iter(&env_path).unwrap() {
        let (name, value) = env.unwrap();

        if std::env::var(&name).is_ok() {
            panic!("Env {name} is already set.");
        }
    }

    dotenv::from_path(&env_path).unwrap();

    Ok(())
}
