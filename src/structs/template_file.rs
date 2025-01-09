use std::{collections::HashMap, path::Path};

use toml::Value;

use super::config::Config;
use crate::{template_file_marker, var_name};

#[derive(Debug)]
pub struct TemplateFile {
    pub name:    &'static str,
    pub marker:  &'static str,
    pub content: &'static str,
}

impl TemplateFile {
    pub const fn new(name: &'static str, marker: &'static str, content: &'static str) -> Self {
        Self {
            name,
            marker,
            content,
        }
    }

    pub fn write(&self, dir: &Path, interpolator: impl Fn(&str) -> String) -> anyhow::Result<()> {
        Self::write_name(self, dir, interpolator, self.name)
    }

    pub fn write_name(
        &self,
        dir: &Path,
        interpolator: impl Fn(&str) -> String,
        name: &str,
    ) -> anyhow::Result<()> {
        std::fs::create_dir_all(dir)?;
        let path = dir.join(name);
        std::fs::write(path, interpolator(self.content))?;
        Ok(())
    }

    pub fn update(&self, dir: &Path, recurse: bool) -> anyhow::Result<()> {
        let path = dir.join(self.name);
        if path.exists() {
            if let Ok(contents) = String::from_utf8(std::fs::read(&path)?) {
                if contents.contains(template_file_marker!(&self.marker)) {
                    let config = Config::load_from_dir(dir.to_path_buf())?;
                    std::fs::write(
                        path,
                        Self::write_vars(&config.templates.vars, self.content.to_string()),
                    )?;
                }
            }
        }
        if recurse {
            for entry in std::fs::read_dir(dir)? {
                let dir = entry?.path();
                if dir.is_dir() {
                    self.update(&dir, recurse)?;
                }
            }
        }

        Ok(())
    }

    pub fn write_vars(vars: &HashMap<String, Value>, mut content: String) -> String {
        for (k, v) in vars {
            let k = var_name!(k);
            let v = match v {
                Value::String(s) => s.to_string(),
                Value::Integer(i) => i.to_string(),
                Value::Boolean(b) => b.to_string(),
                Value::Float(f) => f.to_string(),
                _ => continue,
            };
            content = content.replace(k, &v);
        }
        content
    }
}

#[derive(Debug)]
pub struct TemplateFileWrapper {
    pub dir:  &'static str,
    pub file: &'static TemplateFile,
}

impl TemplateFileWrapper {
    pub const fn new(dir: &'static str, file: &'static TemplateFile) -> Self {
        Self { dir, file }
    }
}
