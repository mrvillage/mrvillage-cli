pub trait Handle {
    fn handle(&self) -> anyhow::Result<()>;
}
