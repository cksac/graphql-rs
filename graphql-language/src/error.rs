// TODO Properly implement!
use std::error;
use std::io;
use std::fmt;

#[derive(Debug)]
pub enum Error {
  Lexer(super::lexer::Error),
  Parser(super::parser::Error),
  Io(io::Error),
}

impl error::Error for Error {
  fn description(&self) -> &str {
    match *self {
      Error::Lexer(_) => "Lexer Error Occurred",
      Error::Parser(_) => "Parser Error Occurred",
      Error::Io(_) => "Io Error Occurred",
    }
  }

  fn cause(&self) -> Option<&error::Error> {
    match *self {
      Error::Lexer(ref e) => Some(e),
      Error::Parser(ref e) => Some(e),
      Error::Io(ref e) => Some(e),
    }
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::Lexer(ref e) => {
        write!(f,
               "{} due to {}",
               error::Error::description(self),
               error::Error::description(e))
      }
      Error::Parser(ref e) => {
        write!(f,
               "{} due to {}",
               error::Error::description(self),
               error::Error::description(e))
      }
      Error::Io(ref e) => {
        write!(f,
               "{} due to {}",
               error::Error::description(self),
               error::Error::description(e))
      }
    }
  }
}

impl From<super::lexer::Error> for Error {
  fn from(error: super::lexer::Error) -> Error {
    Error::Lexer(error)
  }
}

impl From<super::parser::Error> for Error {
  fn from(error: super::parser::Error) -> Error {
    Error::Parser(error)
  }
}

impl From<io::Error> for Error {
  fn from(error: io::Error) -> Error {
    Error::Io(error)
  }
}
