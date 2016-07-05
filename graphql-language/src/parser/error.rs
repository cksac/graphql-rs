// TODO Properly implement!
use std::error;
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum Error {
  Eof, // Should *NEVER* happen!
  UnkownOperation,
  UnexpectedToken,
  MissingExpectedToken,
  ExpectedValueNotFound,
  DuplicateInputObjectField,
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::Eof => write!(f, "End of File!"),
      Error::UnkownOperation => write!(f, "What is this Operation?"),
      Error::UnexpectedToken => write!(f, "This token was not expected!"),
      Error::MissingExpectedToken => write!(f, "There should of been a token here"),
      Error::ExpectedValueNotFound => write!(f, "No value?"),
      Error::DuplicateInputObjectField => write!(f, "Duplicate input object field"),
    }
  }
}

impl error::Error for Error {
  fn description(&self) -> &str {
    match *self {
      Error::Eof => "End of File!",
      Error::UnkownOperation => "What is this OP?",
      Error::UnexpectedToken => "This token was not expected",
      Error::MissingExpectedToken => "There should of been a token here",
      Error::ExpectedValueNotFound => "No value?",
      Error::DuplicateInputObjectField => "Duplicate input object field",
    }
  }

  fn cause(&self) -> Option<&error::Error> {
    None
  }
}
