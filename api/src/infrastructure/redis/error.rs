use bb8_redis::{bb8::RunError, redis::RedisError};

use crate::domain::error::Error;

impl From<RunError<RedisError>> for Error {
    fn from(err: RunError<RedisError>) -> Self {
        Error::External(err.into())
    }
}

impl From<RedisError> for Error {
    fn from(err: RedisError) -> Self {
        Error::External(err.into())
    }
}
