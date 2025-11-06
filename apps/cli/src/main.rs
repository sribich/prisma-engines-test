mod command;
mod logger;
mod path;
mod util;

use command::{debug::DebugCommand, migrate::MigrateCli};

use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

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

    let args = Cli::parse();

    match args.command {
        Command::Debug(debug_command) => command::debug::run(debug_command),
        Command::Migrate(migrate_command) => command::migrate::run(migrate_command).await,
        _ => todo!(),
    }?;

    Ok(())
}
