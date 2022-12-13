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
                let mut config = Config::load();

                let pnw_test = config.ssh.hosts.get_mut("pnw-test");
                if pnw_test.is_none() {
                    return Err(anyhow::anyhow!("SSH host pnw-test not found in config"));
                }
                pnw_test.unwrap().prompt_for_root_if_none("pnw-test");

                let pnw_api = config.ssh.hosts.get_mut("pnw-api");
                if pnw_api.is_none() {
                    return Err(anyhow::anyhow!("SSH host pnw-api not found in config"));
                }
                pnw_api.unwrap().prompt_for_root_if_none("pnw-api");

                let pnw_test = config.ssh.hosts.get("pnw-test").unwrap();
                let pnw_api = config.ssh.hosts.get("pnw-api").unwrap();

                println!("Deploying and refreshing test server cache...");
                ssh_cmd!(pnw_test, "cd ~/api && cap staging deploy || cd /var/vhosts/api/current && echo {} | sudo -S php artisan lighthouse:cache", pnw_test.root_password.as_ref().unwrap())?;

                println!("Refreshing production server cache...");
                ssh_cmd!(
                    pnw_api,
                    "cd /var/vhosts/api/current && echo {} | sudo -S php artisan lighthouse:cache",
                    pnw_api.root_password.as_ref().unwrap()
                )?;

                println!("Done!");

                Ok(())
            },
        }
    }
}
