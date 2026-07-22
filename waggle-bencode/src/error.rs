use serde::de;
use serde::ser;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    Message(String),
    NotSupported(&'static str),
    ExpectedKey,
    ExpectedValue,
    UnsortedKey,
    DuplicateKey,
    Syntax,
    Trailing,
    Eof
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Message(msg) => f.write_str(msg),
            Error::NotSupported(msg) => write!(f, "not supported: {}", msg),
            Error::UnsortedKey => f.write_str("unsorted key"),
            Error::DuplicateKey => f.write_str("duplicate key"),
            Error::ExpectedKey => f.write_str("expected key"),
            Error::ExpectedValue => f.write_str("expected value"),
            Error::Syntax => f.write_str("syntax"),
            Error::Trailing => f.write_str("trailing"),
            Error::Eof => f.write_str("eof")
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Message(value.to_string())
    }
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Message(msg.to_string())
    }
}
