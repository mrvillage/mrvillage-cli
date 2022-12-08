use crate::{include_template_file, structs::template_file::TemplateFile};

pub const PRETTIERRC: TemplateFile =
    TemplateFile::new(".prettierrc", include_template_file!(".prettierrc.toml"));

pub const REMIX_ESLINTRC: TemplateFile = TemplateFile::new(
    ".eslintrc.json",
    include_template_file!("remix.eslintrc.json"),
);

pub const CLIPPY_CONFIG: TemplateFile =
    TemplateFile::new("clippy.toml", include_template_file!("clippy.toml"));

pub const RUSTFMT_CONFIG: TemplateFile =
    TemplateFile::new("rustfmt.toml", include_template_file!("rustfmt.toml"));

pub const CARGO_CONFIG: TemplateFile =
    TemplateFile::new("Cargo.toml", include_template_file!("Cargo.toml"));

pub const RUST_GITIGNORE: TemplateFile =
    TemplateFile::new(".gitignore", include_template_file!("rust.gitignore"));

pub const MRVILLAGE_CONFIG: TemplateFile = TemplateFile::new(
    ".mrvillage.toml",
    include_template_file!("default-mrvillage.toml"),
);
