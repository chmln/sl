#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("source {0} does not exist")]
    SourceDoesNotExist(std::path::PathBuf),
    #[error("target exists. use --force to override")]
    TargetExists
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
