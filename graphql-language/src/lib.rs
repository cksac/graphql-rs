#![allow(unused_variables, unknown_lints)]
pub mod ast;
mod error;
mod lexer;
mod parser;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub fn parse<S: Into<String>>(source: S) -> Result<ast::Root> {
  ast::Root::new(source).parse()
}

use std::path::PathBuf;
pub fn parse_file<S: Into<PathBuf>>(source: S) -> Result<ast::Root> {
  try!(ast::Root::from_file(source)).parse()
}
