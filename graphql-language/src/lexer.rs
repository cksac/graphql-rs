
use std::str::CharIndices;
use std::iter::Peekable;

#[derive(PartialEq, Debug)]
pub enum Token<'a> {
  Eof,
  Punctuator(Punctuator, usize, usize),
  Name(&'a str, usize, usize),
  IntValue(&'a str, usize, usize),
  FloatValue(&'a str, usize, usize),
  StringValue(&'a str, usize, usize),
}

#[derive(PartialEq, Debug)]
pub enum Punctuator {
  Bang,
  Dollar,
  LeftParen,
  RightParen,
  Spread,
  Colon,
  Equals,
  At,
  LeftBracket,
  RightBracket,
  LeftBrace,
  RightBrace,
  Pipe,
}

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
}

fn skip_ignored_token(lexer: &mut Lexer) {
  loop {
    if take!(lexer, '#') {
      take_while_not!(lexer, '\r' | '\n');
    }
    if !take_while!(lexer, '\u{feff}' | ' ' | '\t' | '\r' | '\n' | ',') {
      break;
    }
  }
}

fn scan_spread(lexer: &mut Lexer) -> bool {
  if take!(lexer, '.') {
    if take!(lexer, '.') {
      if take!(lexer, '.') {
        lexer.hi += 1;
        return true;
      }
    }
  }
  return false;
}

fn scan_name(lexer: &mut Lexer) {
  take_while!(lexer, '_' | 'a'...'z' | 'A'...'Z' | '0'...'9');
  take_eof!(lexer);
}

// Return the raw string without unescape character
fn scan_string(lexer: &mut Lexer) -> (bool, bool) {
  let mut is_bad_escape = false;
  let mut is_terminated = false;
  if take!(lexer, '"') {
    lexer.lo += 1;
  }
  loop {
    if take!(lexer, '\\') {
      if !take!(lexer, '\\' | '"' | 'b' | 'f' | 'n' | 'r' | 't' | '/' | 'u') {
        is_bad_escape = true;
        break;
      }
    }
    take_while_not!(lexer, '"' | '\\');
    if take!(lexer, '"') {
      is_terminated = true;
      break;
    }
  }
  (is_terminated, is_bad_escape)
}

// IntValue:   (+|-)?(0|[1-9][0-9]*)
// FloatValue: (+|-)?(0|[1-9][0-9]*)(\.[0-9]+)?((E|e)(+|-)?[0-9]+)?
fn scan_number(lexer: &mut Lexer) -> (bool, bool) {
  let mut is_float = false;
  take!(lexer, '+' | '-');
  // Integer part
  if take!(lexer, '1'...'9') {
    take_while!(lexer, '0'...'9');
    if take!(lexer, ' ' | '\t' | '\r' | '\n' | ',') || take_eof!(lexer) {
      return (true, is_float);
    }
  } else if take!(lexer, '0') {
    if take!(lexer, ' ' | '\t' | '\r' | '\n' | ',') || take_eof!(lexer) {
      return (true, is_float);
    }
  }
  // Fractional part
  if take!(lexer, '.') {
    if !take_while!(lexer, '0'...'9') {
      return (false, is_float);
    }
    is_float = true;
  }
  // Exponent part
  if !is_float && peek!(lexer, 'E' | 'e') {
    return (false, is_float);
  }
  if take!(lexer, 'E' | 'e') {
    if take!(lexer, '+' | '-') {
      if !take_while!(lexer, '0'...'9') {
        return (false, is_float);
      }
    } else {
      take_while!(lexer, '0'...'9');
    }
  }

  if take!(lexer, ' ' | '\t' | '\r' | '\n' | ',') || take_eof!(lexer) {
    return (true, is_float);
  }
  return (false, is_float);
}


impl<'a> Iterator for Lexer<'a> {
  type Item = Result<Token<'a>, &'a str>;
  fn next(&mut self) -> Option<Result<Token<'a>, &'a str>> {
    use self::Token::*;
    use self::Punctuator::*;

    if !self.is_eof && take_eof!(self) {
      self.is_eof = true;
      return Some(Ok(Token::Eof));
    }
    if self.is_eof {
      return None;
    }
    skip_ignored_token(self);

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
          if scan_spread(self) {
            Some(Ok(Punctuator(Spread, self.lo, self.hi)))
          } else {
            Some(Err("Unexpected character."))
          }
        }
        '_' | 'a'...'z' | 'A'...'Z' => {
          scan_name(self);
          Some(Ok(Name(&self.input[self.lo..self.hi], self.lo, self.hi)))
        }
        '-' | '0'...'9' => {
          match scan_number(self) {
            (false, false) => Some(Err("Invalid integer number.")),
            (false, true) => Some(Err("Invalid float number.")),
            (true, false) => Some(Ok(IntValue(&self.input[self.lo..self.hi], self.lo, self.hi))),
            (true, true) => Some(Ok(FloatValue(&self.input[self.lo..self.hi], self.lo, self.hi))),
          }
        }
        '"' => {
          match scan_string(self) {
            (false, _) => Some(Err("Unterminated string.")),
            (_, true) => Some(Err("Bad character escape sequence.")),
            _ => Some(Ok(StringValue(&self.input[self.lo..self.hi], self.lo, self.hi))),
          }
        }
        _ => Some(Err("Unexpected character.")),
      }
    }
    item
  }
}

#[cfg(test)]
mod tests {
  use super::Punctuator::*;
  use super::Token::*;
  use super::{Lexer, Token};

  fn test_next_token(lexer: &mut Lexer, expected: Token) {
    let next = lexer.next();
    assert_eq!(Some(Ok(expected)), next);
  }

  fn test_with_lex(input: &str, expeced: Token) {
    let mut lexer = Lexer::new(&input);
    test_next_token(&mut lexer, expeced);
  }

  fn test_str(input: &str, expeced: &str) {
    let len = expeced.len();
    test_with_lex(input, StringValue(expeced, 1, len + 1))
  }

  #[test]
  fn test_string() {
    test_str(r#""simple""#, r#"simple"#);
    test_str(r#"" white space ""#, r#" white space "#);
    test_str(r#""quote \"""#, r#"quote \""#);
    test_str(r#""slashes \\ \/""#, r#"slashes \\ \/"#);
    test_str(r#""unicode \u1234\u5678\u90AB\uCDEF""#,
             r#"unicode \u1234\u5678\u90AB\uCDEF"#);
  }

  #[test]
  fn test_name() {
    let mut lexer = Lexer::new("     name1    name2   \"simple\" \"  white space   \" other_name");
    test_next_token(&mut lexer, Name("name1", 5, 10));
    test_next_token(&mut lexer, Name("name2", 14, 19));
    test_next_token(&mut lexer, StringValue("simple", 23, 29));
    test_next_token(&mut lexer, StringValue("  white space   ", 32, 48));
    test_next_token(&mut lexer, Name("other_name", 50, 60));
    test_next_token(&mut lexer, Eof);
    assert_eq!(None, lexer.next());
  }

  #[test]
  fn test_number() {
    let mut lexer = Lexer::new("-9 0 9 -0 -9 1234 -1234 0.0 -0.0 1.0 -1.012 -0.101 1.0e2 -1.0E2 \
                                0.00e-002 -0.00E+002 9.1E+0");
    test_next_token(&mut lexer, IntValue("-9", 0, 2));
    test_next_token(&mut lexer, IntValue("0", 3, 4));
    test_next_token(&mut lexer, IntValue("9", 5, 6));
    test_next_token(&mut lexer, IntValue("-0", 7, 9));
    test_next_token(&mut lexer, IntValue("-9", 10, 12));
    test_next_token(&mut lexer, IntValue("1234", 13, 17));
    test_next_token(&mut lexer, IntValue("-1234", 18, 23));
    test_next_token(&mut lexer, FloatValue("0.0", 24, 27));
    test_next_token(&mut lexer, FloatValue("-0.0", 28, 32));
    test_next_token(&mut lexer, FloatValue("1.0", 33, 36));
    test_next_token(&mut lexer, FloatValue("-1.012", 37, 43));
    test_next_token(&mut lexer, FloatValue("-0.101", 44, 50));
    test_next_token(&mut lexer, FloatValue("1.0e2", 51, 56));
    test_next_token(&mut lexer, FloatValue("-1.0E2", 57, 63));
    test_next_token(&mut lexer, FloatValue("0.00e-002", 64, 73));
    test_next_token(&mut lexer, FloatValue("-0.00E+002", 74, 84));
    test_next_token(&mut lexer, FloatValue("9.1E+0", 85, 91));
    test_next_token(&mut lexer, Eof);
    assert_eq!(None, lexer.next());
  }

  #[test]
  fn test_symbol() {
    let mut lexer = Lexer::new("@ !  :$ =    [  {   (|] } )      ...");
    test_next_token(&mut lexer, Punctuator(At, 0, 1));
    test_next_token(&mut lexer, Punctuator(Bang, 2, 3));
    test_next_token(&mut lexer, Punctuator(Colon, 5, 6));
    test_next_token(&mut lexer, Punctuator(Dollar, 6, 7));
    test_next_token(&mut lexer, Punctuator(Equals, 8, 9));
    test_next_token(&mut lexer, Punctuator(LeftBracket, 13, 14));
    test_next_token(&mut lexer, Punctuator(LeftBrace, 16, 17));
    test_next_token(&mut lexer, Punctuator(LeftParen, 20, 21));
    test_next_token(&mut lexer, Punctuator(Pipe, 21, 22));
    test_next_token(&mut lexer, Punctuator(RightBracket, 22, 23));
    test_next_token(&mut lexer, Punctuator(RightBrace, 24, 25));
    test_next_token(&mut lexer, Punctuator(RightParen, 26, 27));
    test_next_token(&mut lexer, Punctuator(Spread, 33, 36));
    test_next_token(&mut lexer, Eof);
    assert_eq!(None, lexer.next());
  }
}
