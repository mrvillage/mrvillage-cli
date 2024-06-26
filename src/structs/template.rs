use std::path::{Path, PathBuf};

use include_dir::{Dir, File};
use tempfile::tempdir;
use xshell::Shell;

use crate::{quiet_cmd, utils::template::flatten_dir};

use super::template_file::TemplateFileWrapper;

#[derive(Debug)]
pub struct Template {
    pub name: &'static str,
    pub dir: Dir<'static>,
    pub config_files: &'static [TemplateFileWrapper],
    pub git: bool,
}

impl Template {
    pub fn write(
        &self,
        dir: PathBuf,
        interpolator: impl Fn(&str) -> String,
        git: Option<(&str, &str)>,
    ) -> anyhow::Result<()> {
        println!("Creating {} project in {}", self.name, dir.display());
        if dir.exists() {
            return Err(anyhow::anyhow!("That directory already exists!"));
        }
        let temp_dir = tempdir()?;
        let res = self.write_files(temp_dir.path(), &interpolator, git);
        if let Err(e) = res {
            temp_dir.close()?;
            Err(e)
        } else {
            std::fs::rename(temp_dir.path(), dir)?;
            std::fs::create_dir(temp_dir.path())?;
            temp_dir.close()?;
            Ok(())
        }
    }

    fn write_files(
        &self,
        dir_path: &Path,
        interpolator: &impl Fn(&str) -> String,
        git: Option<(&str, &str)>,
    ) -> anyhow::Result<()> {
        let dirs = flatten_dir(&self.dir);
        for i in dirs.iter() {
            std::fs::create_dir_all(dir_path.join(i.path()))?;
        }
        let files: Vec<&File> = dirs.into_iter().flat_map(|i| i.files()).collect();
        for i in files {
            let path = dir_path.join(i.path());
            if let Some(contents) = i.contents_utf8() {
                let contents = interpolator(contents);
                std::fs::write(path, contents)?;
            } else {
                std::fs::write(path, i.contents())?;
            }
        }
        for i in self.config_files.iter() {
            i.file.write(&dir_path.join(i.dir), interpolator)?;
        }
        if git.is_some() {
            let sh = Shell::new()?;
            sh.change_dir(dir_path);
            let (org, repo) = git.unwrap();
            quiet_cmd!(sh, "git init").run()?;
            quiet_cmd!(sh, "git remote add origin git@github.com:{org}/{repo}.git").run()?;
        }
        Ok(())
    }
}
