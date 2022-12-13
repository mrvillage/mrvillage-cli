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
        #[arg(short, long)]
        git: Option<bool>,
    },
    #[command(about = "Create a new project with the Rust Binary template")]
    RustBin {
        name: String,
        #[command(flatten)]
        dir: HasDir,
        #[arg(short, long)]
        git: Option<bool>,
    },
    #[command(about = "Create a new project with the Rust Library template")]
    RustLib {
        name: String,
        #[command(flatten)]
        dir: HasDir,
        #[arg(short, long)]
        git: Option<bool>,
    },
    #[command(about = "Create a new project with the Remix with Cloudflare Pages template")]
    RemixPages {
        name: String,
        #[arg(short = 'a', long = "api-domain")]
        api_domain: String,
        #[arg(short = 'n', long = "meta-name")]
        meta_name: String,
        #[arg(short = 'e', long = "meta-description")]
        meta_description: String,
        #[arg(short, long)]
        color: String,
        #[command(flatten)]
        dir: HasDir,
        #[arg(short, long)]
        git: Option<bool>,
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
                Templates::AdventOfCode2022Rust { day, dir, git } => {
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
                    crate::consts::template::ADVENT_OF_CODE_2022_RUST.write(
                        std::path::Path::new(&dir).to_path_buf(),
                        |i| {
                            i.replace(
                                var_name!("crate-name"),
                                format!("day-{}", &day.to_string()).as_str(),
                            )
                        },
                        *git,
                    )
                },
                Templates::RustBin { name, dir, git } => crate::consts::template::RUST_BIN.write(
                    std::path::Path::new(&dir.dir).join(name),
                    |i| i.replace(var_name!("crate-name"), name.as_str()),
                    *git,
                ),
                Templates::RustLib { name, dir, git } => crate::consts::template::RUST_LIB.write(
                    std::path::Path::new(&dir.dir).join(name),
                    |i| i.replace(var_name!("crate-name"), name.as_str()),
                    *git,
                ),
                Templates::RemixPages {
                    name,
                    dir,
                    api_domain,
                    meta_name,
                    meta_description,
                    color,
                    git,
                } => crate::consts::template::REMIX_PAGES.write(
                    std::path::Path::new(&dir.dir).join(name),
                    |i| {
                        i.replace(var_name!("app-name"), name.as_str())
                            .replace(var_name!("api-domain"), api_domain.as_str())
                            .replace(var_name!("meta-tags-name"), meta_name.as_str())
                            .replace(
                                var_name!("meta-tags-description"),
                                meta_description.as_str(),
                            )
                            .replace(var_name!("primary-color"), color.as_str())
                    },
                    *git,
                ),
            },
        }
    }
}
