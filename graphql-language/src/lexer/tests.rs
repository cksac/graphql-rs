
use super::Punctuator::*;
use super::Token::*;
use super::{Lexer, Token, Error};

trait Assert {
  fn next_is_token(&mut self, expected: Token);
  fn next_is_error(&mut self, expected: Error);
}

impl<'a> Assert for Lexer<'a> {
  fn next_is_token(&mut self, expected: Token) {
    assert_eq!(self.next(), Some(Ok(expected)));
  }

  fn next_is_error(&mut self, expected: Error) {
    assert_eq!(self.next(), Some(Err(expected)));
  }
}

fn assert_token(input: &str, expected: Token) {
  let mut lexer = Lexer::new(input);
  lexer.next_is_token(expected);
}

fn assert_error(input: &str, expected: Error) {
  let mut lexer = Lexer::new(input);
  lexer.next_is_error(expected);
}

#[test]
fn disallows_unsupported_control_chars() {
  assert_error("\u{0007}", Error::UnxepectedChar);
}

#[test]
fn accepts_bom_header() {
  assert_token("\u{feff} foo", Name("foo", 4, 7));
}

#[test]
fn skips_ignored_chars() {
  assert_token("

    foo

",
               Name("foo", 6, 9));

  assert_token("
    #comment
    foo#comment
",
               Name("foo", 18, 21));

  assert_token(",,,foo,,,", Name("foo", 3, 6));
  assert_token("", Eof);
}

#[test]
fn lexes_names() {
  assert_token("simple", Name("simple", 0, 6));
  assert_token("Capital", Name("Capital", 0, 7));
  assert_token("camelName", Name("camelName", 0, 9));
  assert_token("snake_name", Name("snake_name", 0, 10));
}

#[test]
fn lexes_bad_names() {
  let mut lexer = Lexer::new("a-b");
  lexer.next_is_token(Name("a", 0, 1));
  lexer.next_is_error(Error::InvalidInt);
  //TODO: fix error cases to advance lexer.iter
  //lexer.next_is_token(Eof);
}

#[test]
fn lexes_string() {
  assert_token("\"simple\"", StringValue("simple".into(), 1, 7));
  assert_token("\" white space \"",
               StringValue(" white space ".into(), 1, 14));
  assert_token("\"quote \\\"\"", StringValue(r#"quote ""#.into(), 1, 9));
  assert_token("\"escaped \\n\\r\\b\\t\\f\"",
               StringValue("escaped \n\r\x08\t\x0c".into(), 1, 19));
  assert_token("\"slashes \\\\ \\/\"",
               StringValue(r#"slashes \ /"#.into(), 1, 14));
  assert_token("\"unicode \\u1234\\u5678\\u90AB\\uCDEF\"",
               StringValue("unicode \u{1234}\u{5678}\u{90ab}\u{cdef}".into(), 1, 33));
  assert_token("\"unicode фы世界\"",
               StringValue("unicode фы世界".into(), 1, 19));
  assert_token("\"фы世界\"", StringValue("фы世界".into(), 1, 11));
}

#[test]
fn lexes_bad_string() {
  assert_error("\"", Error::UnterminatedString);
  assert_error("\"no end quote", Error::UnterminatedString);
  //TODO: fix scan_string
  //assert_error("\"contains unescaped \u{0007} control char\"", Error::UnxepectedChar);
  //assert_error("\"null-byte is not \u{0000} end of file\"", Error::UnxepectedChar);
  assert_error("\"multi\nline\"", Error::UnterminatedString);
  assert_error("\"multi\rline\"", Error::UnterminatedString);
  assert_error("\"bad \\u123", Error::UnterminatedString);
  assert_error("\"bad \\z esc\"", Error::BadEscape);
  assert_error("\"bad \\u1 esc\"", Error::BadUnicodeEscape);
  assert_error("\"bad \\u0XX1 esc\"", Error::BadUnicodeEscape);
  assert_error("\"bad \\uXXXX esc\"", Error::BadUnicodeEscape);
  assert_error("\"bфы世ыы𠱸d \\uXXXF esc\"", Error::BadUnicodeEscape);
}

#[test]
fn lexes_number() {
  assert_token("0", IntValue("0", 0, 1));
  assert_token("9", IntValue("9", 0, 1));
  assert_token("4", IntValue("4", 0, 1));
  assert_token("-4", IntValue("-4", 0, 2));
  assert_token("0.0", FloatValue("0.0", 0, 3));
  assert_token("-0.0", FloatValue("-0.0", 0, 4));
  assert_token("4.123", FloatValue("4.123", 0, 5));
  assert_token("-4.123", FloatValue("-4.123", 0, 6));
  assert_token("0.123", FloatValue("0.123", 0, 5));
  assert_token("0.0123", FloatValue("0.0123", 0, 6));
  assert_token("123e4", FloatValue("123e4", 0, 5));
  assert_token("123E4", FloatValue("123E4", 0, 5));
  assert_token("123E-4", FloatValue("123E-4", 0, 6));
  assert_token("123E+4", FloatValue("123E+4", 0, 6));
  assert_token("-1.123e4", FloatValue("-1.123e4", 0, 8));
  assert_token("-1.123E4", FloatValue("-1.123E4", 0, 8));
  assert_token("-1.123e-4", FloatValue("-1.123e-4", 0, 9));
  assert_token("-1.123E+4", FloatValue("-1.123E+4", 0, 9));
  assert_token("-1.123e4567", FloatValue("-1.123e4567", 0, 11));
  assert_token("1e0", FloatValue("1e0", 0, 3));
  assert_token("0e0", FloatValue("0e0", 0, 3));
  assert_token("1e00", FloatValue("1e00", 0, 4));
  assert_token("1e-0", FloatValue("1e-0", 0, 4));
  assert_token("1e-00", FloatValue("1e-00", 0, 5));
  assert_token("1e+0", FloatValue("1e+0", 0, 4));
  assert_token("1e+00", FloatValue("1e+00", 0, 5));
}

#[test]
fn lexes_bad_number() {
  assert_error("00", Error::InvalidInt);
  assert_error("+1", Error::UnxepectedChar);
  assert_error(".123", Error::UnxepectedChar);
  assert_error("1.", Error::InvalidFloat);
  assert_error("1.A", Error::InvalidFloat);
  assert_error("-A", Error::InvalidInt);
  assert_error("1.0e", Error::InvalidFloat);
  assert_error("1.0e-", Error::InvalidFloat);
  assert_error("1.0e+", Error::InvalidFloat);
  assert_error("1.0eA", Error::InvalidFloat);
}

#[test]
fn lexes_punctuation() {
  assert_token("!", Punctuator(Bang, 0, 1));
  assert_token("$", Punctuator(Dollar, 0, 1));
  assert_token("(", Punctuator(LeftParen, 0, 1));
  assert_token(")", Punctuator(RightParen, 0, 1));
  assert_token(":", Punctuator(Colon, 0, 1));
  assert_token("=", Punctuator(Equals, 0, 1));
  assert_token("@", Punctuator(At, 0, 1));
  assert_token("[", Punctuator(LeftBracket, 0, 1));
  assert_token("]", Punctuator(RightBracket, 0, 1));
  assert_token("{", Punctuator(LeftBrace, 0, 1));
  assert_token("}", Punctuator(RightBrace, 0, 1));
  assert_token("|", Punctuator(Pipe, 0, 1));
  assert_token("...", Punctuator(Spread, 0, 3));
}

#[test]
fn lexes_unexpected_chars() {
  assert_error(".", Error::UnxepectedChar);
  assert_error("..", Error::UnxepectedChar);
  assert_error(".A", Error::UnxepectedChar);
  assert_error("..A", Error::UnxepectedChar);
  assert_error("?", Error::UnxepectedChar);
  assert_error("\u{203B}", Error::UnxepectedChar);
  assert_error("\u{203b}", Error::UnxepectedChar);
  assert_error("ф", Error::UnxepectedChar);
}
