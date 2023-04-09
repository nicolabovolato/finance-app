use crate::domain::error::{AuthErrorType, Error};

impl From<pasetors::errors::Error> for Error {
    fn from(err: pasetors::errors::Error) -> Self {
        match err {
            pasetors::errors::Error::ClaimValidation => Error::Auth(AuthErrorType::InvalidToken),
            pasetors::errors::Error::TokenFormat => Error::Auth(AuthErrorType::InvalidToken),
            pasetors::errors::Error::TokenValidation => Error::Auth(AuthErrorType::InvalidToken),
            _ => Error::External(err.into()),
        }
    }
}
