use std::io;

#[derive(Debug)]
pub enum ParseError {
    Io(io::Error),
    MissingType(String),
    MissingAttribute,
    InvalidFile,
    InvalidTypeName,
    InvalidBlob,
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    ParseError(ParseError),
}

// TODO: change this to just "Result" and make it work like winrt::Result
pub type ParseResult<T> = Result<T, ParseError>;

impl std::convert::From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl std::convert::From<ParseError> for Error {
    fn from(error: ParseError) -> Self {
        Error::ParseError(error)
    }
}

impl std::convert::From<io::Error> for ParseError {
    fn from(error: io::Error) -> Self {
        ParseError::Io(error)
    }
}

impl std::convert::From<ParseError> for std::fmt::Error {
    fn from(_: ParseError) -> Self {
        std::fmt::Error {}
    }
}
