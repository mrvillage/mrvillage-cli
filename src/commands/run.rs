use clap::Subcommand;

use crate::{ssh_cmd, structs::config::Config, traits::handle::Handle};

#[derive(Debug, Subcommand)]
pub enum Actions {
    #[command(about = "Deploy the PnW API")]
    #[command(name = "deploy-pnw-api")]
    DeployPnWApi {
        #[arg(short, long)]
        #[arg(default_value = "false")]
        production: bool,
        #[arg(short, long)]
        #[arg(default_value = "false")]
        staging: bool,
    },
}

impl Handle for Actions {
    fn handle(&self) -> anyhow::Result<()> {
        match self {
            Self::DeployPnWApi {
                production: _,
                staging: _,
            } => {
                let config = Config::load();
                let host = config.ssh.hosts.get("pnw-test").unwrap();
                println!("s");
                println!("{}", ssh_cmd!(host, "echo 1").unwrap());
                Ok(())
            },
        }
    }
}
