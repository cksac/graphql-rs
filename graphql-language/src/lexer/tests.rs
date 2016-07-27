
use super::Punctuator::*;
use super::Token::*;
use super::{Lexer, Token};

fn test_next_token(lexer: &mut Lexer, expected: Token) {
  let next = lexer.next();
  assert_eq!(next, Some(Ok(expected)));
}

fn test_with_lex(input: &str, expeced: Token) {
  let mut lexer = Lexer::new(&input);
  test_next_token(&mut lexer, expeced);
}

#[test]
fn test_string() {
  test_with_lex("\"simple\"", StringValue("simple".to_string(), 1, 7));
  test_with_lex("\" white space \"",
                StringValue(" white space ".to_string(), 1, 14));
  test_with_lex("\"quote \\\"\"", StringValue("quote \"".to_string(), 1, 9));
  test_with_lex("\"escaped \\n\\r\\b\\t\\f\"",
                StringValue("escaped \n\r\x08\t\x0c".to_string(), 1, 19));
  test_with_lex("\"slashes \\\\ \\/\"",
                StringValue("slashes \\ /".to_string(), 1, 14));
  test_with_lex("\"unicode \\u1234\\u5678\\u90AB\\uCDEF\"",
                StringValue("unicode \u{1234}\u{5678}\u{90ab}\u{cdef}".to_string(),
                            1,
                            33));
  test_with_lex("\"Has a фы世界 multi-byte character.\"",
                StringValue("Has a фы世界 multi-byte character.".to_string(), 1, 39));
}

#[test]
fn test_name() {
  let mut lexer = Lexer::new("     name1    name2   \"simple\" \"  white space   \" other_name");
  test_next_token(&mut lexer, Name("name1", 5, 10));
  test_next_token(&mut lexer, Name("name2", 14, 19));
  test_next_token(&mut lexer, StringValue("simple".to_string(), 23, 29));
  test_next_token(&mut lexer,
                  StringValue("  white space   ".to_string(), 32, 48));
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
  let mut lexer = Lexer::new("@ !  :$ =    [  {   (|] } )      ... ...");
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
  test_next_token(&mut lexer, Punctuator(Spread, 37, 40));
  test_next_token(&mut lexer, Eof);
  assert_eq!(None, lexer.next());
}