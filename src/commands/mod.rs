mod projects;
mod run;

use std::process::Command;

use anyhow::Context;
use clap::Subcommand;

use crate::{structs::command::CommandWrapper, traits::handle::Handle};

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "Work with projects")]
    #[command(subcommand)]
    Projects(projects::Commands),
    #[command(about = "Run an action")]
    #[command(subcommand)]
    Run(run::Actions),
    #[command(about = "Run commands concurrently")]
    Parallel {
        #[arg(short, long)]
        commands: Vec<String>,
    },
}

impl Handle for Commands {
    fn handle(&self) -> anyhow::Result<()> {
        match self {
            Self::Projects(projects) => projects.handle(),
            Self::Run(run) => run.handle(),
            Self::Parallel { commands } => {
                if commands.is_empty() {
                    return Err(anyhow::anyhow!("No commands provided"));
                }
                let mut handles = Vec::with_capacity(commands.len());
                for i in commands {
                    let command = if cfg!(target_os = "windows") {
                        let mut command = Command::new("cmd");
                        command.args(["/C", i]);
                        command
                    } else {
                        let mut command = Command::new("sh");
                        command.args(["-c", i]);
                        command
                    };
                    handles.push(
                        CommandWrapper::spawn(command)
                            .with_context(|| format!("Program {i} not found!"))?,
                    );
                }
                for mut i in handles {
                    let _res = i.0.wait();
                }
                Ok(())
            },
        }
    }
}
