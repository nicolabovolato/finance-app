use tracing::{debug, error};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("repository error: {0}")]
    Repository(RepositoryErrorType),
    #[error("auth error: {0}")]
    Auth(AuthErrorType),
    #[error("validation error: {0}")]
    Validation(anyhow::Error),
    #[error(transparent)]
    External(#[from] anyhow::Error),
}

impl Error {
    pub fn log(&self) {
        match self {
            Error::External(_) => error!("{:?}", self),
            _ => debug!("{}", self),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RepositoryErrorType {
    #[error("entity not found")]
    NotFound,
    #[error("entity conflict")]
    Conflict,
}

#[derive(thiserror::Error, Debug)]
pub enum AuthErrorType {
    #[error("invalid otp")]
    InvalidOtp,
    #[error("missing auth")]
    Missing,
    #[error("invalid token")]
    InvalidToken,
}
