
use std::str::CharIndices;
use std::iter::Peekable;

static DEFAULT_SOURCE_NAME: &'static str = "GraphQL";

#[derive(Debug)]
pub struct Source<'a> {
  pub name: &'a str,
  pub body: &'a str,
}

impl<'a> Source<'a> {
  pub fn new(body: &'a str) -> Source<'a> {
    Source {
      name: DEFAULT_SOURCE_NAME,
      body: body,
    }
  }

  pub fn new_with_name(name: &'a str, body: &'a str) -> Source<'a> {
    Source {
      name: name,
      body: body,
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token<'a> {
  Eof,
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
  Name(&'a str),
  Int(&'a str),
  Float(&'a str),
  String(&'a str),
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
    lexer.lo = lexer.lo + 1;
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

// Int:   (+|-)?(0|[1-9][0-9]*)
// Float: (+|-)?(0|[1-9][0-9]*)(\.[0-9]+)?((E|e)(+|-)?[0-9]+)?
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
      self.hi = p;
      item = match c {
        '!' => {
          self.iter.next();
          Some(Ok(Token::Bang))
        }
        '$' => {
          self.iter.next();
          Some(Ok(Token::Dollar))
        }
        '(' => {
          self.iter.next();
          Some(Ok(Token::LeftParen))
        }
        ')' => {
          self.iter.next();
          Some(Ok(Token::RightParen))
        }
        ':' => {
          self.iter.next();
          Some(Ok(Token::Colon))
        }
        '=' => {
          self.iter.next();
          Some(Ok(Token::Equals))
        }
        '@' => {
          self.iter.next();
          Some(Ok(Token::At))
        }
        '[' => {
          self.iter.next();
          Some(Ok(Token::LeftBracket))
        }
        ']' => {
          self.iter.next();
          Some(Ok(Token::RightBracket))
        }
        '{' => {
          self.iter.next();
          Some(Ok(Token::LeftBrace))
        }
        '}' => {
          self.iter.next();
          Some(Ok(Token::RightBrace))
        }
        '|' => {
          self.iter.next();
          Some(Ok(Token::Pipe))
        }
        '.' => {
          if scan_spread(self) {
            Some(Ok(Token::Spread))
          } else {
            Some(Err("Unexpected character."))
          }
        }
        '_' | 'a'...'z' | 'A'...'Z' => {
          scan_name(self);
          Some(Ok(Token::Name(&self.input[self.lo..self.hi])))
        }
        '-' | '0'...'9' => {
          match scan_number(self) {
            (false, false) => Some(Err("Invalid integer number.")),
            (false, true) => Some(Err("Invalid float number.")),
            (true, false) => Some(Ok(Token::Int(&self.input[self.lo..self.hi]))),
            (true, true) => Some(Ok(Token::Float(&self.input[self.lo..self.hi]))),
          }
        }
        '"' => {
          match scan_string(self) {
            (false, _) => Some(Err("Unterminated string.")),
            (_, true) => Some(Err("Bad character escape sequence.")),
            _ => Some(Ok(Token::String(&self.input[self.lo..self.hi]))),
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
  use super::*;

  fn test_next_token(lexer: &mut Lexer, expected: Token) {
    let next = lexer.next();
    assert_eq!(Some(Ok(expected)), next);
  }

  fn test_with_lex(input: &str, expeced: Token) {
    let mut lexer = Lexer::new(&input);
    test_next_token(&mut lexer, expeced);
  }

  fn test_str(input: &str, expeced: &str) {
    test_with_lex(input, Token::String(expeced));
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
    test_next_token(&mut lexer, Token::Name("name1"));
    test_next_token(&mut lexer, Token::Name("name2"));
    test_next_token(&mut lexer, Token::String("simple"));
    test_next_token(&mut lexer, Token::String("  white space   "));
    test_next_token(&mut lexer, Token::Name("other_name"));
    test_next_token(&mut lexer, Token::Eof);
    assert_eq!(None, lexer.next());
  }

  #[test]
  fn test_number() {
    let mut lexer = Lexer::new("-9 0 9 -0 -9 1234 -1234 0.0 -0.0 1.0 -1.012 -0.101 1.0e2 -1.0E2 \
                                0.00e-002 -0.00E+002 9.1E+0");
    test_next_token(&mut lexer, Token::Int("-9"));
    test_next_token(&mut lexer, Token::Int("0"));
    test_next_token(&mut lexer, Token::Int("9"));
    test_next_token(&mut lexer, Token::Int("-0"));
    test_next_token(&mut lexer, Token::Int("-9"));
    test_next_token(&mut lexer, Token::Int("1234"));
    test_next_token(&mut lexer, Token::Int("-1234"));
    test_next_token(&mut lexer, Token::Float("0.0"));
    test_next_token(&mut lexer, Token::Float("-0.0"));
    test_next_token(&mut lexer, Token::Float("1.0"));
    test_next_token(&mut lexer, Token::Float("-1.012"));
    test_next_token(&mut lexer, Token::Float("-0.101"));
    test_next_token(&mut lexer, Token::Float("1.0e2"));
    test_next_token(&mut lexer, Token::Float("-1.0E2"));
    test_next_token(&mut lexer, Token::Float("0.00e-002"));
    test_next_token(&mut lexer, Token::Float("-0.00E+002"));
    test_next_token(&mut lexer, Token::Float("9.1E+0"));
    test_next_token(&mut lexer, Token::Eof);
    assert_eq!(None, lexer.next());
  }
}
