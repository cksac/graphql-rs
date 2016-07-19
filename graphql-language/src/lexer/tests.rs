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
  let len = expeced.chars().count();
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
fn test_skip() {
  let mut lexer = Lexer::new("\t\t\r\n\n\n#this is a skipp!\n  name1");
  test_next_token(&mut lexer, Name("name1", 26, 31));
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
