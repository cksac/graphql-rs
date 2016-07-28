use std::str::CharIndices;
use std::iter::Peekable;
use std::result;

#[cfg(test)]
mod tests;

#[macro_use]
mod macros;

mod error;
pub use self::error::Error;
pub type Result<T> = result::Result<T, Error>;

#[derive(PartialEq, Debug)]
pub enum Token<'a> {
  Eof,
  Punctuator(Punctuator, usize, usize),
  Name(&'a str, usize, usize),
  IntValue(&'a str, usize, usize),
  FloatValue(&'a str, usize, usize),
  StringValue(String, usize, usize),
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

pub struct Lexer<'a> {
  input: &'a str,
  iter: Peekable<CharIndices<'a>>,
  eof_emmited: bool,
  lo: usize,
  hi: usize,
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Lexer<'a> {
    Lexer {
      input: input,
      iter: input.char_indices().peekable(),
      eof_emmited: false,
      lo: 0,
      hi: 0,
    }
  }
}

impl<'a> Iterator for Lexer<'a> {
  type Item = Result<Token<'a>>;
  fn next(&mut self) -> Option<Result<Token<'a>>> {
    if self.eof_emmited {
      None
    } else if self.iter.peek().is_none() {
      self.eof_emmited = true;
      Some(Ok(Token::Eof))
    } else {
      skip_ignored_token(self);
      let &(p, c) = self.iter.peek().unwrap();
      self.lo = p;
      Some(match c {
        '!' => punctuator!(self, Bang),
        '$' => punctuator!(self, Dollar),
        '(' => punctuator!(self, LeftParen),
        ')' => punctuator!(self, RightParen),
        ':' => punctuator!(self, Colon),
        '=' => punctuator!(self, Equals),
        '@' => punctuator!(self, At),
        '[' => punctuator!(self, LeftBracket),
        ']' => punctuator!(self, RightBracket),
        '{' => punctuator!(self, LeftBrace),
        '}' => punctuator!(self, RightBrace),
        '|' => punctuator!(self, Pipe),
        '.' => scan_spread(self),
        '_' | 'a'...'z' | 'A'...'Z' => scan_name(self),
        '-' | '0'...'9' => scan_number(self),
        '"' => scan_string(self),
        _ => Err(Error::UnxepectedChar),
      })
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

fn scan_spread<'a>(lexer: &mut Lexer<'a>) -> Result<Token<'a>> {
  if take!(lexer, '.') && take!(lexer, '.') && take!(lexer, '.') {
    Ok(Token::Punctuator(Punctuator::Spread, lexer.lo, lexer.hi))
  } else {
    Err(Error::UnxepectedChar)
  }
}

fn scan_name<'a>(lexer: &mut Lexer<'a>) -> Result<Token<'a>> {
  take_while!(lexer, '_' | 'a'...'z' | 'A'...'Z' | '0'...'9');
  Ok(Token::Name(&lexer.input[lexer.lo..lexer.hi], lexer.lo, lexer.hi))
}

fn scan_number<'a>(lexer: &mut Lexer<'a>) -> Result<Token<'a>> {
  take!(lexer, '-');
  // Integer part
  if take!(lexer, '1'...'9') {
    take_while!(lexer, '0'...'9');
    if follow_by!(lexer, ' ' | '\t' | '\r' | '\n' | ',') || lexer.iter.peek().is_none() {
      return Ok(Token::IntValue(&lexer.input[lexer.lo..lexer.hi], lexer.lo, lexer.hi));
    }
  } else if take!(lexer, '0') {
    if follow_by!(lexer, ' ' | '\t' | '\r' | '\n' | ',') || lexer.iter.peek().is_none() {
      return Ok(Token::IntValue(&lexer.input[lexer.lo..lexer.hi], lexer.lo, lexer.hi));
    }
  }
  if !peek!(lexer, '.' | 'E' | 'e') {
    return Err(Error::InvalidInt);
  }
  // Fractional part
  if take!(lexer, '.') {
    if !take_while!(lexer, '0'...'9') {
      return Err(Error::InvalidFloat);
    }
  }
  // Exponent part
  if take!(lexer, 'E' | 'e') {
    take!(lexer, '+' | '-');
    if !take_while!(lexer, '0'...'9') {
      return Err(Error::InvalidFloat);
    }
  }
  if follow_by!(lexer, ' ' | '\t' | '\r' | '\n' | ',') || lexer.iter.peek().is_none() {
    return Ok(Token::FloatValue(&lexer.input[lexer.lo..lexer.hi], lexer.lo, lexer.hi));
  }
  Err(Error::InvalidFloat)
}

fn scan_string<'a>(lexer: &mut Lexer<'a>) -> Result<Token<'a>> {
  if take!(lexer, '"') {
    lexer.lo += 1;
  }
  loop {
    if take!(lexer, '\\') {
      if !take!(lexer, '\\' | '"' | 'b' | 'f' | 'n' | 'r' | 't' | '/' | 'u') {
        return Err(Error::BadEscape);
      }
    }
    if !take_while_not!(lexer, '"' | '\\' | '\r' | '\n') && !peek!(lexer, '"' | '\\') {
      return Err(Error::UnterminatedString);
    }
    if peek!(lexer, '\r' | '\n') {
      return Err(Error::UnterminatedString);
    }
    if peek!(lexer, '"') {
      lexer.iter.next();
      let s = try!(unexcape_str(&lexer.input[lexer.lo..lexer.hi]));
      return Ok(Token::StringValue(s, lexer.lo, lexer.hi));
    }
  }
}

fn unexcape_str(s: &str) -> Result<String> {
  let mut buf = String::with_capacity(s.len());
  let mut p = s.chars().peekable();
  while let Some(c) = p.next() {
    match c {
      '\\' => {
        match p.next() {
          Some('\\') => buf.push('\\'),
          Some('/') => buf.push('/'),
          Some('"') => buf.push('"'),
          Some('n') => buf.push('\n'),
          Some('r') => buf.push('\r'),
          Some('t') => buf.push('\t'),
          Some('b') => buf.push('\x08'),
          Some('f') => buf.push('\x0c'),
          Some('u') => {
            let u1 = try!(p.next().ok_or(Error::BadUnicodeEscape));
            let u2 = try!(p.next().ok_or(Error::BadUnicodeEscape));
            let u3 = try!(p.next().ok_or(Error::BadUnicodeEscape));
            let u4 = try!(p.next().ok_or(Error::BadUnicodeEscape));
            let s = try!(u32::from_str_radix(format!("{}{}{}{}", u1, u2, u3, u4).as_ref(), 16)
              .or(Err(Error::BadUnicodeEscape)));
            let ch = try!(::std::char::from_u32(s).ok_or(Error::InvalidUtfChar));
            buf.push(ch);
          }
          _ => return Err(Error::UnxepectedChar),
        }
      }
      _ => buf.push(c),
    }
  }
  Ok(buf)
}
