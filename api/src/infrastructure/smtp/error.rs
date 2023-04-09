use crate::domain::error::Error;

impl From<lettre::error::Error> for Error {
    fn from(err: lettre::error::Error) -> Self {
        Error::External(err.into())
    }
}

impl From<lettre::address::AddressError> for Error {
    fn from(err: lettre::address::AddressError) -> Self {
        Error::External(err.into())
    }
}

impl From<lettre::transport::smtp::Error> for Error {
    fn from(err: lettre::transport::smtp::Error) -> Self {
        Error::External(err.into())
    }
}
