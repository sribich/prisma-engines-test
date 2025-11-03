mod command;

use command::{debug::DebugCommand, migrate::MigrateCli};

use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use schema_context::load_schema_context;
// use schema_config::load_schema_files;

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

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    println!("{:#?}", load_schema_context(None, None));

    let args = Cli::parse();

    match args.command {
        Command::Debug(debug_command) => command::debug::run(debug_command),
        Command::Migrate(migrate_command) => command::migrate::run(migrate_command),
        _ => todo!(),
    }?;

    Ok(())
}
