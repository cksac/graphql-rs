#![allow(unused_variables, dead_code, unused_mut, unused_imports)]
pub mod ast;
mod error;
mod lexer;
mod parser;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub fn parse<'a>(source: &'a str) -> Result<ast::Root<'a>> {
  ast::Root::new(source).parse()
}

pub fn parse_file<'a>(source: &'a std::path::Path) -> Result<ast::Root<'a>> {
  try!(ast::Root::from_file(source)).parse()
}
