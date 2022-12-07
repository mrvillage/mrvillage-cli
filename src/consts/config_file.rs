use crate::{include_config_file, structs::config_file::ConfigFile};

pub static PRETTIERRC: ConfigFile =
    ConfigFile::new(".prettierrc", include_config_file!(".prettierrc"));

pub const REMIX_ESLINTRC: ConfigFile = ConfigFile::new(
    ".eslintrc.json",
    include_config_file!("remix.eslintrc.json"),
);

pub const CLIPPY_CONFIG: ConfigFile =
    ConfigFile::new("clippy.toml", include_config_file!("clippy.toml"));

pub const RUSTFMT_CONFIG: ConfigFile =
    ConfigFile::new("rustfmt.toml", include_config_file!("rustfmt.toml"));
