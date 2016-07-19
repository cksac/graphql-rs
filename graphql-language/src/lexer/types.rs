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
