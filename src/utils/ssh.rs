use anyhow::Result;

use crate::structs::config::Host;

#[macro_export]
macro_rules! ssh_cmd {
    ($host:expr, $cmd:literal) => {
        $crate::utils::ssh::ssh_command($host, &format!("{}", std::format_args!($cmd)))
    };
    ($host:expr, $cmd:literal, $($args:tt)*) => {
        $crate::utils::ssh::ssh_command($host, &format!("{}", std::format_args!($cmd, $($args)*)))
    }
}

pub fn ssh_command(host: &Host, command: &str) -> Result<String> {
    let mut ssh = std::process::Command::new("ssh");
    if host.forward_agent {
        ssh.arg("-A");
    }
    println!("{} {}", host.connection_string(), command);
    ssh.arg("-p")
        .arg(host.port.to_string())
        .arg(host.connection_string())
        .arg(command);
    Ok(String::from_utf8(ssh.output()?.stdout)?)
}
