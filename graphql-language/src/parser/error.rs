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
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::Eof => write!(f, "bad"),
      Error::UnkownOperation => write!(f, "bad"),
      Error::UnexpectedToken => write!(f, "bad"),
      Error::MissingExpectedToken => write!(f, "bad"),
      Error::ExpectedValueNotFound => write!(f, "No value?"),
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
    }
  }

  fn cause(&self) -> Option<&error::Error> {
    None
  }
}