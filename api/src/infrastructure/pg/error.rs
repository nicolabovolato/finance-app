use crate::domain::error::{Error, RepositoryErrorType};

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::Database(e) => match e.code().unwrap_or_default().as_ref() {
                "23505" => Error::Repository(RepositoryErrorType::Conflict),
                _ => Error::External(e.into()),
            },
            sqlx::Error::RowNotFound => Error::Repository(RepositoryErrorType::NotFound),
            _ => Error::External(err.into()),
        }
    }
}
