use std::path::Path;

#[derive(Debug)]
pub struct TemplateFile {
    pub name: &'static str,
    pub content: &'static str,
}

impl TemplateFile {
    pub const fn new(name: &'static str, content: &'static str) -> Self {
        Self { name, content }
    }

    pub fn write(&self, dir: &Path, content: String) -> anyhow::Result<()> {
        let path = dir.join(&self.name);
        std::fs::write(path, content)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct TemplateFileWrapper {
    pub dir: &'static str,
    pub file: &'static TemplateFile,
}

impl TemplateFileWrapper {
    pub const fn new(dir: &'static str, file: &'static TemplateFile) -> Self {
        Self { dir, file }
    }
}
