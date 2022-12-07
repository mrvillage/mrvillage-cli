use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectConfigError {
    #[error(
        "The project config directory for {0} could not be found. Please create the required directory at {1}"
    )]
    ProjectConfigDirectoryNotFound(&'static str, &'static str),
}
