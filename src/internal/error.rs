use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    UnsupportedSaveVersion,
    InvalidContainer(&'static str),
    NotImplemented(&'static str),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnsupportedSaveVersion => f.write_str("unsupported save version"),
            Error::InvalidContainer(message) => write!(f, "invalid container: {message}"),
            Error::NotImplemented(message) => write!(f, "not implemented: {message}"),
        }
    }
}

impl std::error::Error for Error {}
