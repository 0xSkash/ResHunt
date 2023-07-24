use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    InvalidPath,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Error::InvalidPath => "The provided path is invalid",
        };

        write!(f, "{message}")
    }
}

impl std::error::Error for Error {}