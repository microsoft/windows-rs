use std::fmt::{Display, Formatter};

/// An error encountered while reading an ECMA-335 metadata image.
#[derive(Debug)]
pub enum Error {
    /// The input could not be read.
    Io(std::io::Error),
    /// The input is structurally invalid at the given byte offset.
    Invalid {
        /// The byte offset at which validation failed.
        offset: usize,
        /// A short description of the violated structural rule.
        message: &'static str,
    },
    /// A required metadata stream is absent.
    MissingStream(&'static str),
    /// A metadata stream name appears more than once.
    DuplicateStream(String),
}

impl Error {
    pub(crate) fn invalid(offset: usize, message: &'static str) -> Self {
        Self::Invalid { offset, message }
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => error.fmt(formatter),
            Self::Invalid { offset, message } => {
                write!(formatter, "invalid metadata at byte {offset}: {message}")
            }
            Self::MissingStream(name) => write!(formatter, "missing metadata stream `{name}`"),
            Self::DuplicateStream(name) => {
                write!(formatter, "duplicate metadata stream `{name}`")
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}
