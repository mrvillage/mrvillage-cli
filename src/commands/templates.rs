use std::{
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use clap::{Args, Subcommand};

use crate::{
    consts::{
        template::{DEFAULT_LEPTOS, TS_LIB},
        template_files::{CLIPPY_CONFIG, PRETTIERRC, REMIX_ESLINTRC, RUSTFMT_CONFIG},
        DEFAULT_ORG,
    },
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
        #[arg(short, long)]
        bin: Option<String>,
        #[arg(short, long)]
        repo: Option<String>,
        #[arg(short, long)]
        org: Option<String>,
    },
    #[command(about = "Create a new project with the Rust Library template")]
    RustLib {
        name: String,
        #[command(flatten)]
        dir: HasDir,
        #[arg(short, long)]
        git: Option<bool>,
        #[arg(short, long)]
        repo: Option<String>,
        #[arg(short, long)]
        org: Option<String>,
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
    #[command(about = "Create a new project with the default Leptos template")]
    Leptos {
        name: String,
        #[command(flatten)]
        dir: HasDir,
        #[arg(short, long)]
        git: Option<bool>,
        #[arg(short, long)]
        repo: Option<String>,
        #[arg(short, long)]
        org: Option<String>,
    },
    #[command(about = "Create a new project with the default TypeScript library templates")]
    #[command(name = "ts-lib")]
    TsLib {
        name: String,
        #[command(flatten)]
        dir: HasDir,
        #[arg(short, long)]
        git: Option<bool>,
        #[arg(short, long)]
        repo: Option<String>,
        #[arg(short, long)]
        org: Option<String>,
        #[arg(short, long)]
        pkg_name: String,
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

fn replace_rust_params(
    text: &str,
    name: &str,
    bin: &Option<String>,
    repo: &Option<String>,
    org: &Option<String>,
) -> String {
    replace_default_params(text, name, repo, org)
        .replace(var_name!("crate-name"), name)
        .replace(
            var_name!("bin-name"),
            bin.as_ref().map(|i| i.as_str()).unwrap_or(name),
        )
}

fn replace_ts_params(
    text: &str,
    name: &str,
    repo: &Option<String>,
    org: &Option<String>,
    pkg_name: &str,
) -> String {
    replace_default_params(text, name, repo, org).replace(var_name!("package-name"), pkg_name)
}

fn replace_default_params(
    text: &str,
    name: &str,
    repo: &Option<String>,
    org: &Option<String>,
) -> String {
    text.replace(
        var_name!("repo-name"),
        repo.as_ref().map(|i| i.as_str()).unwrap_or(name),
    )
    .replace(
        var_name!("org-name"),
        org.as_ref().map(|i| i.as_str()).unwrap_or("mrvillage"),
    )
    .replace(
        var_name!("current-year"),
        &(1970
            + (SystemTime::now().duration_since(UNIX_EPOCH))
                .expect("system time error")
                .as_secs()
                / (365 * 24 * 60 * 60))
            .to_string(),
    )
}

impl Handle for Commands {
    fn handle(&self) -> anyhow::Result<()> {
        match self {
            Self::New(command) => match command {
                TemplatesNew::AdventOfCode2022Rust { day, dir, .. } => {
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
                        None,
                    )
                },
                TemplatesNew::RustBin {
                    name,
                    dir,
                    git,
                    bin,
                    repo,
                    org,
                } => crate::consts::template::RUST_BIN.write(
                    std::path::Path::new(&dir.dir).join(name),
                    |i| replace_rust_params(i, name, bin, repo, org),
                    if git.unwrap_or(crate::consts::template::RUST_BIN.git) {
                        Some((
                            org.as_ref().map(|x| x.as_str()).unwrap_or(DEFAULT_ORG),
                            repo.as_ref().map(|x| x.as_str()).unwrap_or(name.as_str()),
                        ))
                    } else {
                        None
                    },
                ),
                TemplatesNew::RustLib {
                    name,
                    dir,
                    git,
                    repo,
                    org,
                } => crate::consts::template::RUST_LIB.write(
                    std::path::Path::new(&dir.dir).join(name),
                    |i| replace_rust_params(i, name, &None, repo, org),
                    if git.unwrap_or(crate::consts::template::RUST_LIB.git) {
                        Some((
                            org.as_ref().map(|x| x.as_str()).unwrap_or(DEFAULT_ORG),
                            repo.as_ref().map(|x| x.as_str()).unwrap_or(name.as_str()),
                        ))
                    } else {
                        None
                    },
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
                    if git.unwrap_or(crate::consts::template::REMIX_PAGES.git) {
                        Some((DEFAULT_ORG, name))
                    } else {
                        None
                    },
                ),
                TemplatesNew::Leptos {
                    name,
                    dir,
                    git,
                    repo,
                    org,
                } => DEFAULT_LEPTOS.write(
                    std::path::Path::new(&dir.dir).join(name),
                    |i| replace_rust_params(i, name, &None, repo, org),
                    if git.unwrap_or(crate::consts::template::DEFAULT_LEPTOS.git) {
                        Some((
                            org.as_ref().map(|x| x.as_str()).unwrap_or(DEFAULT_ORG),
                            repo.as_ref().map(|x| x.as_str()).unwrap_or(name.as_str()),
                        ))
                    } else {
                        None
                    },
                ),
                TemplatesNew::TsLib {
                    name,
                    dir,
                    git,
                    repo,
                    org,
                    pkg_name,
                } => TS_LIB.write(
                    std::path::Path::new(&dir.dir).join(name),
                    |i| replace_ts_params(i, name, repo, org, pkg_name),
                    if git.unwrap_or(crate::consts::template::TS_LIB.git) {
                        Some((
                            org.as_ref().map(|x| x.as_str()).unwrap_or(DEFAULT_ORG),
                            repo.as_ref().map(|x| x.as_str()).unwrap_or(name.as_str()),
                        ))
                    } else {
                        None
                    },
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
