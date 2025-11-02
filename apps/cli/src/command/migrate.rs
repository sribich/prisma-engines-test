use anyhow::Result;
use clap::{Parser, Subcommand};

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

pub fn run(args: MigrateCli) -> Result<()> {
    Ok(())
}
