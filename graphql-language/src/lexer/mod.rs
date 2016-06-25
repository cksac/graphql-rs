mod error;
pub use self::error::Error;
use std::result;
pub type Result<T> = result::Result<T, Error>;

mod types;
pub use self::types::Punctuator;
pub use self::types::Token;

#[cfg(test)]
mod tests;

use std::str::CharIndices;
use std::iter::Peekable;

macro_rules! take {
  ($lexer: ident, $($p: pat)|*) => ({
    let mut is_taken = false;
    if let Some(&(p, c)) = $lexer.iter.peek() {
      match c {
        $(
          $p => {
            $lexer.iter.next();
            $lexer.hi = p;
            is_taken = true;
          }
        )*
        _ => {}
      }
    }
    is_taken
  });
}

macro_rules! take_while {
  ($lexer: ident, $($p: pat)|*) => ({
    let mut is_taken = false;
    while let Some(&(p, c)) = $lexer.iter.peek() {
      match c {
        $(
          $p => {
            $lexer.iter.next();
            is_taken = true;
          }
        )*
        _ => {
          $lexer.hi = p;
          break;
        }
      }
    }
    is_taken
  });
}

macro_rules! take_while_not {
  ($lexer: ident, $($p: pat)|*) => ({
    let mut is_taken = false;
    while let Some(&(p, c)) = $lexer.iter.peek() {
      match c {
        $(
          $p => {
            $lexer.hi = p;
            break;
          }
        )*
        _ => {
            $lexer.iter.next();
            is_taken = true;
        }
      }
    }
    is_taken
  });
}

macro_rules! peek {
  ($lexer: ident, $($p: pat)|*) => ({
    let mut is_match = false;
    if let Some(&(_, c)) = $lexer.iter.peek() {
      match c {
        $(
          $p => {
            is_match = true;
          }
        )*
        _ => {}
      }
    }
    is_match
  });
}

macro_rules! take_eof {
  ($lexer: ident) => ({
    if $lexer.iter.peek().is_none() {
      $lexer.hi = $lexer.input_len;
      true
    } else {
      false
    }
  });
}

pub struct Lexer<'a> {
  input: &'a str,
  input_len: usize,
  iter: Peekable<CharIndices<'a>>,
  is_eof: bool,
  lo: usize,
  hi: usize,
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Lexer<'a> {
    Lexer {
      input: input,
      input_len: input.len(),
      iter: input.char_indices().peekable(),
      is_eof: false,
      lo: 0,
      hi: 0,
    }
  }

  fn skip_ignored_token(&mut self) {
    loop {
      if take!(self, '#') {
        take_while_not!(self, '\r' | '\n');
      }
      if !take_while!(self, '\u{feff}' | ' ' | '\t' | '\r' | '\n' | ',') {
        break;
      }
    }
  }

  fn scan_spread(&mut self) -> bool {
    if take!(self, '.') {
      if take!(self, '.') {
        if take!(self, '.') {
          self.hi += 1;
          return true;
        }
      }
    }
    return false;
  }

  fn scan_name(&mut self) {
    take_while!(self, '_' | 'a'...'z' | 'A'...'Z' | '0'...'9');
    take_eof!(self);
  }

  // Return the raw string without unescape character
  fn scan_string(&mut self) -> (bool, bool) {
    let mut is_bad_escape = false;
    let mut is_terminated = false;
    if take!(self, '"') {
      self.lo += 1;
    }
    loop {
      if take!(self, '\\') {
        if !take!(self, '\\' | '"' | 'b' | 'f' | 'n' | 'r' | 't' | '/' | 'u') {
          is_bad_escape = true;
          break;
        }
      }
      take_while_not!(self, '"' | '\\');
      if take!(self, '"') {
        is_terminated = true;
        break;
      }
    }
    (is_terminated, is_bad_escape)
  }

  // IntValue:   (+|-)?(0|[1-9][0-9]*)
  // FloatValue: (+|-)?(0|[1-9][0-9]*)(\.[0-9]+)?((E|e)(+|-)?[0-9]+)?
  fn scan_number(&mut self) -> (bool, bool) {
    let mut is_float = false;
    take!(self, '+' | '-');
    // Integer part
    if take!(self, '1'...'9') {
      take_while!(self, '0'...'9');
      if take!(self, ' ' | '\t' | '\r' | '\n' | ',') || take_eof!(self) {
        return (true, is_float);
      }
    } else if take!(self, '0') {
      if take!(self, ' ' | '\t' | '\r' | '\n' | ',') || take_eof!(self) {
        return (true, is_float);
      }
    }
    // Fractional part
    if take!(self, '.') {
      if !take_while!(self, '0'...'9') {
        return (false, is_float);
      }
      is_float = true;
    }
    // Exponent part
    if !is_float && peek!(self, 'E' | 'e') {
      return (false, is_float);
    }
    if take!(self, 'E' | 'e') {
      if take!(self, '+' | '-') {
        if !take_while!(self, '0'...'9') {
          return (false, is_float);
        }
      } else {
        take_while!(self, '0'...'9');
      }
    }

    if take!(self, ' ' | '\t' | '\r' | '\n' | ',') || take_eof!(self) {
      return (true, is_float);
    }
    return (false, is_float);
  }
}

impl<'a> Iterator for Lexer<'a> {
  type Item = Result<Token<'a>>;
  fn next(&mut self) -> Option<Result<Token<'a>>> {
    use self::Token::*;
    use self::Punctuator::*;

    if !self.is_eof && take_eof!(self) {
      self.is_eof = true;
      return Some(Ok(Token::Eof));
    }
    if self.is_eof {
      return None;
    }
    self.skip_ignored_token();

    let mut item = None;
    if let Some(&(p, c)) = self.iter.peek() {
      self.lo = p;
      self.hi = p + 1;
      item = match c {
        '!' => {
          self.iter.next();
          Some(Ok(Punctuator(Bang, self.lo, self.hi)))
        }
        '$' => {
          self.iter.next();
          Some(Ok(Punctuator(Dollar, self.lo, self.hi)))
        }
        '(' => {
          self.iter.next();
          Some(Ok(Punctuator(LeftParen, self.lo, self.hi)))
        }
        ')' => {
          self.iter.next();
          Some(Ok(Punctuator(RightParen, self.lo, self.hi)))
        }
        ':' => {
          self.iter.next();
          Some(Ok(Punctuator(Colon, self.lo, self.hi)))
        }
        '=' => {
          self.iter.next();
          Some(Ok(Punctuator(Equals, self.lo, self.hi)))
        }
        '@' => {
          self.iter.next();
          Some(Ok(Punctuator(At, self.lo, self.hi)))
        }
        '[' => {
          self.iter.next();
          Some(Ok(Punctuator(LeftBracket, self.lo, self.hi)))
        }
        ']' => {
          self.iter.next();
          Some(Ok(Punctuator(RightBracket, self.lo, self.hi)))
        }
        '{' => {
          self.iter.next();
          Some(Ok(Punctuator(LeftBrace, self.lo, self.hi)))
        }
        '}' => {
          self.iter.next();
          Some(Ok(Punctuator(RightBrace, self.lo, self.hi)))
        }
        '|' => {
          self.iter.next();
          Some(Ok(Punctuator(Pipe, self.lo, self.hi)))
        }
        '.' => {
          if self.scan_spread() {
            Some(Ok(Punctuator(Spread, self.lo, self.hi)))
          } else {
            Some(Err(Error::UnxepectedChar))
          }
        }
        '_' | 'a'...'z' | 'A'...'Z' => {
          self.scan_name();
          Some(Ok(Name(&self.input[self.lo..self.hi], self.lo, self.hi)))
        }
        '-' | '0'...'9' => {
          match self.scan_number() {
            (false, false) => Some(Err(Error::InvalidInt)),
            (false, true) => Some(Err(Error::InvalidFloat)),
            (true, false) => Some(Ok(IntValue(&self.input[self.lo..self.hi], self.lo, self.hi))),
            (true, true) => Some(Ok(FloatValue(&self.input[self.lo..self.hi], self.lo, self.hi))),
          }
        }
        '"' => {
          match self.scan_string() {
            (false, _) => Some(Err(Error::UnterminatedString)),
            (_, true) => Some(Err(Error::BadEscape)),
            _ => Some(Ok(StringValue(&self.input[self.lo..self.hi], self.lo, self.hi))),
          }
        }
        _ => Some(Err(Error::UnxepectedChar)),
      }
    }
    item
  }
}
