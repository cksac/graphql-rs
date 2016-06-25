// TODO Properly implement!
use std::error;
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum Error {
  UnxepectedChar,
  InvalidInt,
  InvalidFloat,
  UnterminatedString,
  BadEscape,
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::UnxepectedChar => write!(f, "bad"),
      Error::InvalidInt => write!(f, "bad"),
      Error::InvalidFloat => write!(f, "bad"),
      Error::UnterminatedString => write!(f, "bad"),
      Error::BadEscape => write!(f, "bad"),
    }
  }
}

impl error::Error for Error {
  fn description(&self) -> &str {
    match *self {
      Error::UnxepectedChar => "Unexpected character",
      Error::InvalidInt => "Invalid integer number",
      Error::InvalidFloat => "Invalid float number",
      Error::UnterminatedString => "Unterminated string",
      Error::BadEscape => "Bad character escape sequence",
    }
  }

  fn cause(&self) -> Option<&error::Error> {
    None
  }
}
