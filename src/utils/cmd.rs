#[macro_export]
macro_rules! quiet_cmd {
    ($sh:expr, $cmd:literal) => {
        xshell::cmd!($sh, $cmd)
            .quiet()
            .ignore_stderr()
            .ignore_stdout()
    };
}
