mod commands;
pub mod consts;
pub mod enums;
pub mod structs;
pub mod traits;
pub mod utils;

use anyhow::Result;
use clap::Parser;
use commands::Commands;
use traits::handle::Handle;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    cli.command.handle()?;

    Ok(())
}
