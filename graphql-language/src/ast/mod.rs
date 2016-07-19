#![allow(doc_markdown)]
//! AST node definitions which will be parsed into. Based off of the
//! `graphql-js` [`ast.js`][1] definitions.
//!
//! [1]: https://github.com/graphql/graphql-js/blob/dfe676c3011efe9560b9fa0fcbd2b7bd87476d02/src/language/ast.js

/// All AST node types implement this trait.
pub trait Node {
  fn location(&self) -> Option<&Location>;
}

macro_rules! impl_node_for {
  ($data:ident) => {
    impl Node for $data {
      fn location(&self) -> Option<&Location> {
        self.loc.as_ref()
      }
    }
  }
}

use super::Result;
use std::path::PathBuf;
use super::parser::Parser;

/// The root of the document, contains the Source and the Document node.
pub struct Root {
  pub source: String,
  pub document: Document,
}

impl Root {
  pub fn new<S: Into<String>>(src: S) -> Self {
    Root {
      source: src.into(),
      document: Document {
        loc: None,
        definitions: Vec::new(),
      },
    }
  }

  pub fn from_file<S: Into<PathBuf>>(src: S) -> Result<Self> {
    use std::fs::File;
    use std::io::Read;

    let path = src.into();
    let mut f = try!(File::open(path.clone()));
    let mut buf = String::new();
    try!(f.read_to_string(&mut buf));
    Ok(Self::new(buf))
  }

  // TODO PARSE
  pub fn parse(mut self) -> Result<Self> {
    self.document = try!(Parser::parse(&self.source));
    Ok(self)
  }
}

/// Contains some character offsets that identify where the source of the AST
/// is from.
pub struct Location {
  pub start: usize,
  pub end: usize,
}

/// Document : Definition+
pub struct Document {
  pub loc: Option<Location>,
  pub definitions: Vec<Definition>,
}

impl_node_for! { Document }

/// Directives : Directive+
pub type Directives = Vec<Directive>;

/// Directive : @ Name Arguments?
pub struct Directive {
  pub loc: Option<Location>,
  pub name: Name,
  pub arguments: Option<Arguments>,
}

impl_node_for! { Directive }

mod definitions;
pub use self::definitions::*;
mod types;
pub use self::types::*;
mod selections;
pub use self::selections::*;
mod values;
pub use self::values::*;
