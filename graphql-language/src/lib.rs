#![allow(unused_variables)]
pub mod ast;
mod error;
mod source;
mod lexer;
mod parser;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

use source::Source;

pub fn parse<'a>(source: &'a str) -> Result<ast::Root<'a>> {
  parse_source(Source::new(source))
}

pub fn parse_file<'a>(source: &'a std::path::Path) -> Result<ast::Root<'a>> {
  parse_source(try!(Source::from_file(source)))
}

// TODO: Check if maybe useable outside of this
fn parse_source<'a>(source: Source<'a>) -> Result<ast::Root<'a>> {
  let root = ast::Root::new(source);
  try!(root.parse());
  Ok(root)
}
