use clap::Subcommand;

use crate::traits::handle::Handle;

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
                // TODO: implement .mrvillage directory and allow storing IPs for servers or see if the libssh-rs crate is safe and figure out how to use it instead
                Ok(())
            },
        }
    }
}
