use std::path::{Path, PathBuf};

fn main() {
    let mut out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    out_dir.push("templates");
    if out_dir.exists() {
        std::fs::remove_dir_all(&out_dir).unwrap();
    }
    std::fs::create_dir(&out_dir).unwrap();
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let templates_dir = manifest_dir.join("templates");
    copy_dir(&templates_dir, &out_dir).unwrap();
}

fn copy_dir(from: &Path, to: &Path) -> Result<(), std::io::Error> {
    for entry in std::fs::read_dir(from)? {
        let path = entry?.path();
        if path.file_name().is_none() {
            continue;
        }
        let to = to.join(path.file_name().unwrap());
        if path.is_dir() {
            std::fs::create_dir(&to)?;
            copy_dir(&path, &to)?;
        } else {
            std::fs::copy(&path, &to)?;
        }
    }
    Ok(())
}
