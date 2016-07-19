// TODO Properly implement!
use std::error;
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum Error {
  UnexpectedEof, // Should *NEVER* happen!
  UnkownOperation,
  UnexpectedToken,
  MissingExpectedToken,
  ExpectedValueNotFound,
  DuplicateInputObjectField,
  Lexer(super::super::lexer::Error),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::UnexpectedEof => write!(f, "End of File reached before it was expected!"),
      Error::UnkownOperation => write!(f, "What is this Operation?"),
      Error::UnexpectedToken => write!(f, "This token was not expected!"),
      Error::MissingExpectedToken => write!(f, "There should of been a token here"),
      Error::ExpectedValueNotFound => write!(f, "No value?"),
      Error::DuplicateInputObjectField => write!(f, "Duplicate input object field"),
      Error::Lexer(ref e) => e.fmt(f),
    }
  }
}

impl error::Error for Error {
  fn description(&self) -> &str {
    match *self {
      Error::UnexpectedEof => "End of File reached before it was expected!",
      Error::UnkownOperation => "What is this OP?",
      Error::UnexpectedToken => "This token was not expected",
      Error::MissingExpectedToken => "There should of been a token here",
      Error::ExpectedValueNotFound => "No value?",
      Error::DuplicateInputObjectField => "Duplicate input object field",
      Error::Lexer(ref e) => e.description(),
    }
  }

  fn cause(&self) -> Option<&error::Error> {
    match *self {
      Error::Lexer(ref c) => Some(c),
      _ => None,
    }
  }
}

impl From<super::super::lexer::Error> for Error {
  fn from(error: super::super::lexer::Error) -> Error {
    Error::Lexer(error)
  }
}
