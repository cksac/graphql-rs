use std::error;
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum Error {
  UnxepectedChar,
  InvalidInt,
  InvalidFloat,
  UnterminatedString,
  BadEscape,
  BadUnicodeEscape,
  InvalidUtfChar,
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::UnxepectedChar => write!(f, "Unexpected character"),
      Error::InvalidInt => write!(f, "Invalid integer number"),
      Error::InvalidFloat => write!(f, "Invalid float number"),
      Error::UnterminatedString => write!(f, "Unterminated string"),
      Error::BadEscape => write!(f, "Bad character escape sequence"),
      Error::BadUnicodeEscape => write!(f, "Bad unicode escape sequence"),
      Error::InvalidUtfChar => write!(f, "Invalid UTF-8 character"),
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
      Error::BadUnicodeEscape => "Bad unicode escape sequence",
      Error::InvalidUtfChar => "Invalid UTF-8 character",
    }
  }

  fn cause(&self) -> Option<&error::Error> {
    None
  }
}
