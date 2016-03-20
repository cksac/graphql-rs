
use std::str::CharIndices;
use std::iter::Peekable;

#[derive(PartialEq, Debug)]
pub enum Token<'a> {
  Eof(usize, usize),
  Bang(usize, usize),
  Dollar(usize, usize),
  LeftParen(usize, usize),
  RightParen(usize, usize),
  Spread(usize, usize),
  Colon(usize, usize),
  Equals(usize, usize),
  At(usize, usize),
  LeftBracket(usize, usize),
  RightBracket(usize, usize),
  LeftBrace(usize, usize),
  RightBrace(usize, usize),
  Pipe(usize, usize),
  Name(usize, usize, &'a str),
  Int(usize, usize, &'a str),
  Float(usize, usize, &'a str),
  String(usize, usize, &'a str),
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
      return Some(Ok(Token::Eof(self.lo, self.hi)));
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
          Some(Ok(Token::Bang(self.lo, self.hi)))
        }
        '$' => {
          self.iter.next();
          Some(Ok(Token::Dollar(self.lo, self.hi)))
        }
        '(' => {
          self.iter.next();
          Some(Ok(Token::LeftParen(self.lo, self.hi)))
        }
        ')' => {
          self.iter.next();
          Some(Ok(Token::RightParen(self.lo, self.hi)))
        }
        ':' => {
          self.iter.next();
          Some(Ok(Token::Colon(self.lo, self.hi)))
        }
        '=' => {
          self.iter.next();
          Some(Ok(Token::Equals(self.lo, self.hi)))
        }
        '@' => {
          self.iter.next();
          Some(Ok(Token::At(self.lo, self.hi)))
        }
        '[' => {
          self.iter.next();
          Some(Ok(Token::LeftBracket(self.lo, self.hi)))
        }
        ']' => {
          self.iter.next();
          Some(Ok(Token::RightBracket(self.lo, self.hi)))
        }
        '{' => {
          self.iter.next();
          Some(Ok(Token::LeftBrace(self.lo, self.hi)))
        }
        '}' => {
          self.iter.next();
          Some(Ok(Token::RightBrace(self.lo, self.hi)))
        }
        '|' => {
          self.iter.next();
          Some(Ok(Token::Pipe(self.lo, self.hi)))
        }
        '.' => {
          if scan_spread(self) {
            Some(Ok(Token::Spread(self.lo, self.hi)))
          } else {
            Some(Err("Unexpected character."))
          }
        }
        '_' | 'a'...'z' | 'A'...'Z' => {
          scan_name(self);
          Some(Ok(Token::Name(self.lo, self.hi, &self.input[self.lo..self.hi])))
        }
        '-' | '0'...'9' => {
          match scan_number(self) {
            (false, false) => Some(Err("Invalid integer number.")),
            (false, true) => Some(Err("Invalid float number.")),
            (true, false) => Some(Ok(Token::Int(self.lo, self.hi, &self.input[self.lo..self.hi]))),
            (true, true) => Some(Ok(Token::Float(self.lo, self.hi, &self.input[self.lo..self.hi]))),
          }
        }
        '"' => {
          match scan_string(self) {
            (false, _) => Some(Err("Unterminated string.")),
            (_, true) => Some(Err("Bad character escape sequence.")),
            _ => Some(Ok(Token::String(self.lo, self.hi, &self.input[self.lo..self.hi]))),
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
    let len = expeced.len();
    test_with_lex(input, Token::String(1, len + 1, expeced))
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
    test_next_token(&mut lexer, Token::Name(5, 10,"name1"));
    test_next_token(&mut lexer, Token::Name(14,19,"name2"));
    test_next_token(&mut lexer, Token::String(23,29,"simple"));
    test_next_token(&mut lexer, Token::String(32,48,"  white space   "));
    test_next_token(&mut lexer, Token::Name(50,60,"other_name"));
    test_next_token(&mut lexer, Token::Eof(50,60));
    assert_eq!(None, lexer.next());
  }

  #[test]
  fn test_number() {
    let mut lexer = Lexer::new("-9 0 9 -0 -9 1234 -1234 0.0 -0.0 1.0 -1.012 -0.101 1.0e2 -1.0E2 \
                                0.00e-002 -0.00E+002 9.1E+0");
    test_next_token(&mut lexer, Token::Int(0,2,    "-9"));
    test_next_token(&mut lexer, Token::Int(3,4,    "0"));
    test_next_token(&mut lexer, Token::Int(5,6,    "9"));
    test_next_token(&mut lexer, Token::Int(7,9,    "-0"));
    test_next_token(&mut lexer, Token::Int(10,12,  "-9"));
    test_next_token(&mut lexer, Token::Int(13,17,  "1234"));
    test_next_token(&mut lexer, Token::Int(18,23,  "-1234"));
    test_next_token(&mut lexer, Token::Float(24,27,"0.0"));
    test_next_token(&mut lexer, Token::Float(28,32,"-0.0"));
    test_next_token(&mut lexer, Token::Float(33,36,"1.0"));
    test_next_token(&mut lexer, Token::Float(37,43,"-1.012"));
    test_next_token(&mut lexer, Token::Float(44,50,"-0.101"));
    test_next_token(&mut lexer, Token::Float(51,56,"1.0e2"));
    test_next_token(&mut lexer, Token::Float(57,63,"-1.0E2"));
    test_next_token(&mut lexer, Token::Float(64,73,"0.00e-002"));
    test_next_token(&mut lexer, Token::Float(74,84,"-0.00E+002"));
    test_next_token(&mut lexer, Token::Float(85,91,"9.1E+0"));
    test_next_token(&mut lexer, Token::Eof(85,91));
    assert_eq!(None, lexer.next());
  }

  #[test]
  fn test_symbol() {
    let mut lexer = Lexer::new("@ !  :$ =    [  {   (|] } )      ...");
    test_next_token(&mut lexer, Token::At(0,0));
    test_next_token(&mut lexer, Token::Bang(2,2));
    test_next_token(&mut lexer, Token::Colon(5,5));
    test_next_token(&mut lexer, Token::Dollar(6,6));
    test_next_token(&mut lexer, Token::Equals(8,8));
    test_next_token(&mut lexer, Token::LeftBracket(13,13));
    test_next_token(&mut lexer, Token::LeftBrace(16,16));
    test_next_token(&mut lexer, Token::LeftParen(20,20));
    test_next_token(&mut lexer, Token::Pipe(21,21));
    test_next_token(&mut lexer, Token::RightBracket(22,22));
    test_next_token(&mut lexer, Token::RightBrace(24,24));
    test_next_token(&mut lexer, Token::RightParen(26,26));
    test_next_token(&mut lexer, Token::Spread(33,35));
    test_next_token(&mut lexer, Token::Eof(33,36));
    assert_eq!(None, lexer.next());
  }
}
