use clap::Subcommand;
use xshell::Shell;

use crate::{quiet_cmd, ssh_cmd, structs::config::Config, traits::handle::Handle};

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
    #[command(about = "Deploy PnW")]
    #[command(name = "deploy-pnw")]
    DeployPnW {
        #[arg(short, long)]
        #[arg(default_value = "false")]
        production: bool,
        #[arg(short, long)]
        #[arg(default_value = "false")]
        staging: bool,
    },
    #[command(about = "Delete empty files")]
    #[command(name = "delete-empty-files")]
    DeleteEmptyFiles {
        #[arg(short, long)]
        #[arg(default_value = ".")]
        dir: String,
    },
}

impl Handle for Actions {
    fn handle(&self) -> anyhow::Result<()> {
        match self {
            Self::DeployPnWApi {
                production,
                staging,
            } => {
                let production = *production;
                let staging = *staging;

                if !staging && !production {
                    return Err(anyhow::anyhow!(
                        "You must deploy to at least one environment"
                    ));
                }

                let mut config = Config::load()?;

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

                let deploy = if production && staging {
                    "cap production deploy || cap staging deploy"
                } else if production {
                    "cap production deploy"
                } else {
                    "cap staging deploy"
                };

                if staging {
                    println!("Deploying and refreshing test server cache...");
                } else {
                    println!("Deploying...");
                }
                ssh_cmd!(
                    pnw_test,
                    "cd ~/api && {deploy}{}",
                    if staging {
                        format!(" || cd /var/vhosts/api/current && echo {} | sudo -S php artisan lighthouse:cache",  pnw_test.root_password.as_ref().unwrap())
                    } else {
                        "".into()
                    }
                )?;

                if production {
                    println!("Refreshing production server cache...");
                    ssh_cmd!(
                    pnw_api,
                    "cd /var/vhosts/api/current && echo {} | sudo -S php artisan lighthouse:cache",
                    pnw_api.root_password.as_ref().unwrap()
                )?;
                }

                println!("Done!");

                Ok(())
            },
            Self::DeployPnW {
                production,
                staging,
            } => {
                let production = *production;
                let staging = *staging;

                if !staging && !production {
                    return Err(anyhow::anyhow!(
                        "You must deploy to at least one environment"
                    ));
                }

                let mut config = Config::load()?;

                let pnw_test = config.ssh.hosts.get_mut("pnw-test");
                if pnw_test.is_none() {
                    return Err(anyhow::anyhow!("SSH host pnw-test not found in config"));
                }
                pnw_test.unwrap().prompt_for_root_if_none("pnw-test");

                let pnw_test = config.ssh.hosts.get("pnw-test").unwrap();

                let deploy = if production && staging {
                    "cap production deploy || cap staging deploy"
                } else if production {
                    "cap production deploy"
                } else {
                    "cap staging deploy"
                };

                println!("Deploying...");
                ssh_cmd!(pnw_test, "cd ~/main-site && {deploy}")?;

                println!("Done!");

                Ok(())
            },
            Self::DeleteEmptyFiles { dir } => {
                let sh = Shell::new()?;
                quiet_cmd!(sh, "find {dir} -type f -empty -print -delete").run()?;
                println!("Deleted empty files in {dir}!");
                Ok(())
            },
        }
    }
}
