use include_dir::include_dir;

use crate::structs::{config_file::ConfigFileWrapper, template::Template};

use super::config_file::{CLIPPY_CONFIG, RUSTFMT_CONFIG};

pub static ADVENT_OF_CODE_2022_RUST_CONFIG: Template = Template {
    name: "aoc22-rs",
    dir: include_dir!("$CARGO_MANIFEST_DIR/templates/advent_of_code_2022_rust"),
    config_files: &[
        ConfigFileWrapper::new("", &CLIPPY_CONFIG),
        ConfigFileWrapper::new("", &RUSTFMT_CONFIG),
    ],
};
