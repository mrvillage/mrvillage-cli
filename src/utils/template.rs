use include_dir::Dir;

#[macro_export]
macro_rules! var_name {
    ($v:expr) => {
        &format!("$$-cli-{}$$", $v)
    };
}

pub fn flatten_dir<'a>(dir: &'a Dir) -> Vec<&'a Dir<'a>> {
    let mut dirs = vec![dir];
    for i in dir.dirs() {
        dirs.append(&mut flatten_dir(i));
    }
    dirs
}

pub fn interpolate_default_vars(content: &str) -> String {
    content.to_string()
}