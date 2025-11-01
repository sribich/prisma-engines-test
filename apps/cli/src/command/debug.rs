use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "workspace")]
pub struct DebugCommand {
    /// Custom path to the schema.prisma file
    #[arg(long)]
    schema: Option<String>,
}

pub fn run(args: DebugCommand) -> Result<()> {
    Ok(())
}
