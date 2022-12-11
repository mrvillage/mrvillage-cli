use crate::{structs::config::Config, traits::handle::Handle};
use clap::Subcommand;
use std::{fmt::Write, io::Write as IoWrite};

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "View the current config file hierarchy")]
    Where,
}

impl Handle for Commands {
    fn handle(&self) -> anyhow::Result<()> {
        match self {
            Self::Where => {
                let global_path = Config::global_path();
                let mut configs = Config::config_paths();
                let mut buffer = String::new();
                let mut ticker = 0;
                writeln!(buffer, "The following configuration files are being used to build the configuration used when running commands in this directory.")?;
                writeln!(buffer, "Configuration options applied in files higher on the hierarchy override options specified lower down.")?;
                while let Some(path) = configs.pop_back() {
                    ticker += 1;
                    writeln!(buffer, "({ticker}) {}", path.display())?;
                }
                ticker += 1;
                writeln!(buffer, "Global ({ticker}). {}", global_path.display())?;
                std::io::stdout().write_all(buffer.as_bytes())?;
                Ok(())
            },
        }
    }
}
