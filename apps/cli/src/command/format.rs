use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(name = "workspace")]
pub struct FormatCli {
    /// Custom path to the schema.prisma file
    #[arg(long)]
    schema: Option<String>,
}

pub fn run(args: FormatCli) -> Result<()> {
    Ok(())
}
