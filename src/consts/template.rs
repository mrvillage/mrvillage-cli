use include_dir::include_dir;

use crate::structs::{template::Template, template_file::TemplateFileWrapper};

use super::template_files::{
    CARGO_CONFIG, CHANGELOG_MD, CLIPPY_CONFIG, LICENSE, MRVILLAGE_CONFIG, PRETTIERRC,
    RELEASE_ACTION_BINARY, RELEASE_ACTION_LIBRARY, RELEASE_TOML, REMIX_ESLINTRC, RUSTFMT_CONFIG,
    RUST_ACTION, RUST_GITIGNORE,
};

pub const ADVENT_OF_CODE_2022_RUST: Template = Template {
    name: "aoc22-rs",
    dir: include_dir!("$CARGO_MANIFEST_DIR/templates/advent_of_code_2022_rust"),
    config_files: &[
        TemplateFileWrapper::new("", &CLIPPY_CONFIG),
        TemplateFileWrapper::new("", &RUSTFMT_CONFIG),
        TemplateFileWrapper::new("", &CARGO_CONFIG),
        TemplateFileWrapper::new("", &RUST_GITIGNORE),
        TemplateFileWrapper::new("", &MRVILLAGE_CONFIG),
    ],
    git: false,
};

pub const RUST_BIN: Template = Template {
    name: "default-rs",
    dir: include_dir!("$CARGO_MANIFEST_DIR/templates/default_rust_bin"),
    config_files: &[
        TemplateFileWrapper::new("", &CLIPPY_CONFIG),
        TemplateFileWrapper::new("", &RUSTFMT_CONFIG),
        TemplateFileWrapper::new("", &CARGO_CONFIG),
        TemplateFileWrapper::new("", &RUST_GITIGNORE),
        TemplateFileWrapper::new("", &MRVILLAGE_CONFIG),
        TemplateFileWrapper::new(".github/workflows", &RELEASE_ACTION_BINARY),
        TemplateFileWrapper::new("", &RELEASE_TOML),
        TemplateFileWrapper::new("", &CHANGELOG_MD),
        TemplateFileWrapper::new("", &LICENSE),
        TemplateFileWrapper::new(".github/workflows", &RUST_ACTION),
    ],
    git: true,
};

pub const RUST_LIB: Template = Template {
    name: "default-rs",
    dir: include_dir!("$CARGO_MANIFEST_DIR/templates/default_rust_lib"),
    config_files: &[
        TemplateFileWrapper::new("", &CLIPPY_CONFIG),
        TemplateFileWrapper::new("", &RUSTFMT_CONFIG),
        TemplateFileWrapper::new("", &CARGO_CONFIG),
        TemplateFileWrapper::new("", &RUST_GITIGNORE),
        TemplateFileWrapper::new("", &MRVILLAGE_CONFIG),
        TemplateFileWrapper::new(".github/workflows", &RELEASE_ACTION_LIBRARY),
        TemplateFileWrapper::new("", &RELEASE_TOML),
        TemplateFileWrapper::new("", &CHANGELOG_MD),
        TemplateFileWrapper::new("", &LICENSE),
        TemplateFileWrapper::new(".github/workflows", &RUST_ACTION),
    ],
    git: true,
};

pub const REMIX_PAGES: Template = Template {
    name: "remix-pages",
    dir: include_dir!("$CARGO_MANIFEST_DIR/templates/remix_pages"),
    config_files: &[
        TemplateFileWrapper::new("", &PRETTIERRC),
        TemplateFileWrapper::new("", &REMIX_ESLINTRC),
        TemplateFileWrapper::new("", &MRVILLAGE_CONFIG),
    ],
    git: true,
};

pub const DEFAULT_LEPTOS: Template = Template {
    name: "default-leptos",
    dir: include_dir!("$CARGO_MANIFEST_DIR/templates/default_leptos"),
    config_files: &[
        TemplateFileWrapper::new("", &CLIPPY_CONFIG),
        TemplateFileWrapper::new("", &RUSTFMT_CONFIG),
        TemplateFileWrapper::new(".github/workflows", &RUST_ACTION),
    ],
    git: true,
};

pub const TS_LIB: Template = Template {
    name: "ts-lib",
    dir: include_dir!("$CARGO_MANIFEST_DIR/templates/ts_lib"),
    config_files: &[
        TemplateFileWrapper::new("", &MRVILLAGE_CONFIG),
        TemplateFileWrapper::new("", &PRETTIERRC),
        TemplateFileWrapper::new("", &LICENSE),
    ],
    git: true,
};
