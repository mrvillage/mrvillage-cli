use clap::{Args, Subcommand};

use crate::{traits::handle::Handle, var_name};

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "Create a project")]
    #[command(subcommand)]
    New(Templates),
}

#[derive(Debug, Subcommand)]
pub enum Templates {
    #[command(about = "Create a new project with the Advent of Code 2022 Rust template")]
    #[command(name = "aoc22-rs")]
    AdventOfCode2022Rust {
        day: u8,
        #[arg(short, long)]
        dir: Option<String>,
    },
}

#[derive(Debug, Args)]
pub struct HasDir {
    #[arg(
        short,
        long,
        default_value_t = std::env::current_dir().unwrap().display().to_string()
    )]
    dir: String,
}

impl Handle for Commands {
    fn handle(&self) -> anyhow::Result<()> {
        match self {
            Self::New(command) => match command {
                Templates::AdventOfCode2022Rust { day, dir } => {
                    let dir = if let Some(dir) = dir {
                        format!(
                            "{}/{}/{}",
                            std::env::current_dir().unwrap().display(),
                            dir,
                            day
                        )
                    } else {
                        format!("{}/day-{}", std::env::current_dir().unwrap().display(), day)
                    };
                    crate::consts::template::ADVENT_OF_CODE_2022_RUST_CONFIG
                        .write(std::path::Path::new(&dir), |i| {
                            i.replace(var_name!("day"), &day.to_string())
                        })
                },
            },
        }
    }
}
