use include_dir::include_dir;

use crate::structs::{template::Template, template_file::TemplateFileWrapper};

use super::template_files::{
    CARGO_CONFIG, CLIPPY_CONFIG, MRVILLAGE_CONFIG, PRETTIERRC, REMIX_ESLINTRC, RUSTFMT_CONFIG,
    RUST_GITIGNORE,
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
