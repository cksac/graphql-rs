pub mod ast;
mod source;
mod lexer;
mod parser;

use ast::Document;
use source::{Source, from_file};

pub fn parse<'a>(input: &'a str) -> Document<'a> {
  parse_source(Source::new(input))
}

use std::path::Path;

pub fn parse_file<'a>(file: &'a Path, buf: &'a mut String) -> Document<'a> {
  parse_source(from_file(file, buf).unwrap()) // FIXME
}

fn parse_source<'a>(src: Source<'a>) -> Document<'a> {
  let mut res = Document::new(src);

  let parser = parser::Parser::new(&res.source);

  for def in parser {
    res.add_definition(def.unwrap());// FIXME
  }

  res
}
