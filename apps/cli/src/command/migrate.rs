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
    match args.command {
        MigrateCommand::Deploy => todo!(),
        MigrateCommand::Dev => todo!(),
        MigrateCommand::Diff => todo!(),
        MigrateCommand::Reset => todo!(),
        MigrateCommand::Resolve => todo!(),
        MigrateCommand::Status => migrate_status(),
    }
}

fn migrate_status() -> Result<()> {
    Ok(())
}
