#[derive(Debug)]
pub struct CommandWrapper(pub std::process::Child);

impl CommandWrapper {
    pub fn spawn(mut command: std::process::Command) -> anyhow::Result<Self> {
        Ok(Self(command.spawn()?))
    }
}

impl Drop for CommandWrapper {
    fn drop(&mut self) {
        let _res = self.0.kill();
    }
}
