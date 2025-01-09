use crate::{include_template_file, structs::template_file::TemplateFile};

pub const PRETTIERRC: TemplateFile = TemplateFile::new(
    ".prettierrc.toml",
    "prettierrc",
    include_template_file!(".prettierrc.toml"),
);

pub const REMIX_ESLINTRC: TemplateFile = TemplateFile::new(
    ".eslintrc.json",
    "remix-eslintrc",
    include_template_file!("remix.eslintrc.json"),
);

pub const CLIPPY_CONFIG: TemplateFile = TemplateFile::new(
    "clippy.toml",
    "clippy",
    include_template_file!("clippy.toml"),
);

pub const RUSTFMT_CONFIG: TemplateFile = TemplateFile::new(
    "rustfmt.toml",
    "rustfmt",
    include_template_file!("rustfmt.toml"),
);

pub const CARGO_CONFIG: TemplateFile = TemplateFile::new(
    "Cargo.toml",
    "",
    include_template_file!("template.Cargo.toml"),
);

pub const RUST_GITIGNORE: TemplateFile =
    TemplateFile::new(".gitignore", "", include_template_file!("rust.gitignore"));

pub const MRVILLAGE_CONFIG: TemplateFile = TemplateFile::new(
    ".mrvillage.toml",
    "",
    include_template_file!("default-mrvillage.toml"),
);

pub const RELEASE_ACTION_BINARY: TemplateFile = TemplateFile::new(
    "post-release.yaml",
    "post-release-bin",
    include_template_file!("post-release-bin.yaml"),
);

pub const RELEASE_ACTION_LIBRARY: TemplateFile = TemplateFile::new(
    "post-release.yaml",
    "post-release-lib",
    include_template_file!("post-release-lib.yaml"),
);

pub const RELEASE_TOML: TemplateFile = TemplateFile::new(
    "release.toml",
    "release",
    include_template_file!("release.toml"),
);

pub const CHANGELOG_MD: TemplateFile = TemplateFile::new(
    "CHANGELOG.md",
    "changelog",
    include_template_file!("CHANGELOG.md"),
);

pub const LICENSE: TemplateFile =
    TemplateFile::new("LICENSE", "", include_template_file!("LICENSE"));

pub const RUST_ACTION: TemplateFile = TemplateFile::new(
    "rust.yaml",
    "rust-action",
    include_template_file!("rust-action.yaml"),
);

pub const TEX_DOC: TemplateFile =
    TemplateFile::new("main.tex", "tex-doc", include_template_file!("tex-doc.tex"));
