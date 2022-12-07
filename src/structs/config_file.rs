use std::path::Path;

#[derive(Debug)]
pub struct ConfigFile {
    pub name: &'static str,
    pub content: &'static str,
}

impl ConfigFile {
    pub const fn new(name: &'static str, content: &'static str) -> Self {
        Self { name, content }
    }

    pub fn write(&self, dir: &Path) -> anyhow::Result<()> {
        let path = dir.join(&self.name);
        std::fs::write(path, self.content)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct ConfigFileWrapper {
    pub dir: &'static str,
    pub file: &'static ConfigFile,
}

impl ConfigFileWrapper {
    pub const fn new(dir: &'static str, file: &'static ConfigFile) -> Self {
        Self { dir, file }
    }
}
