mod projects;

use clap::Subcommand;

use crate::traits::handle::Handle;

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "Work with projects")]
    #[command(subcommand)]
    Projects(projects::Commands),
}

impl Handle for Commands {
    fn handle(&self) -> anyhow::Result<()> {
        match self {
            Self::Projects(projects) => projects.handle(),
        }
    }
}
