use std::path::Path;

use clap::{Args, Subcommand};

use crate::{
    consts::template_files::{CLIPPY_CONFIG, PRETTIERRC, REMIX_ESLINTRC, RUSTFMT_CONFIG},
    traits::handle::Handle,
    var_name,
};

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "Use a template")]
    #[command(subcommand)]
    New(TemplatesNew),
    #[command(about = "Update files from a template")]
    #[command(subcommand)]
    Update(TemplatesUpdate),
}

#[derive(Debug, Subcommand)]
pub enum TemplatesNew {
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

#[derive(Debug, Subcommand)]
pub enum TemplatesUpdate {
    #[command(about = "Update clippy.toml files")]
    Clippy {
        #[arg(short, long, default_value = "false")]
        recurse: bool,
        #[command(flatten)]
        dir: HasDir,
    },
    #[command(about = "Update .prettierrc.toml files")]
    Prettier {
        #[arg(short, long, default_value = "false")]
        recurse: bool,
        #[command(flatten)]
        dir: HasDir,
    },
    #[command(about = "Update Remix .eslintrc.json files")]
    RemixEslint {
        #[arg(short, long, default_value = "false")]
        recurse: bool,
        #[command(flatten)]
        dir: HasDir,
    },
    #[command(about = "Update rustfmt.toml files")]
    Rustfmt {
        #[arg(short, long, default_value = "false")]
        recurse: bool,
        #[command(flatten)]
        dir: HasDir,
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
                TemplatesNew::AdventOfCode2022Rust { day, dir, git } => {
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
                TemplatesNew::RustBin { name, dir, git } => crate::consts::template::RUST_BIN
                    .write(
                        std::path::Path::new(&dir.dir).join(name),
                        |i| i.replace(var_name!("crate-name"), name.as_str()),
                        *git,
                    ),
                TemplatesNew::RustLib { name, dir, git } => crate::consts::template::RUST_LIB
                    .write(
                        std::path::Path::new(&dir.dir).join(name),
                        |i| i.replace(var_name!("crate-name"), name.as_str()),
                        *git,
                    ),
                TemplatesNew::RemixPages {
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
            Self::Update(command) => match command {
                TemplatesUpdate::Clippy { recurse, dir } => {
                    CLIPPY_CONFIG.update(Path::new(&dir.dir), *recurse)
                },
                TemplatesUpdate::Prettier { recurse, dir } => {
                    PRETTIERRC.update(Path::new(&dir.dir), *recurse)
                },
                TemplatesUpdate::RemixEslint { recurse, dir } => {
                    REMIX_ESLINTRC.update(Path::new(&dir.dir), *recurse)
                },
                TemplatesUpdate::Rustfmt { recurse, dir } => {
                    RUSTFMT_CONFIG.update(Path::new(&dir.dir), *recurse)
                },
            },
        }
    }
}
