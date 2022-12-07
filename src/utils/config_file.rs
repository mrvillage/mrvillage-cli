#[macro_export]
macro_rules! include_config_file {
    ($v:expr) => {
        include_str!(concat!("$CARGO_MANIFEST_DIR/../../../config_files/", $v))
    };
}
