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

macro_rules! impl_life_node_for {
  ($data:ident) => {
    impl<'a> Node for $data<'a> {
      fn location(&self) -> Option<&Location> {
        self.loc.as_ref()
      }
    }
  }
}

use std::borrow::Cow;
use super::Result;
use std::path::Path;

/// The root of the document, contains the Source and the Document node.
pub struct Root<'a> {
  pub source: Cow<'a, str>,
  pub document: Document<'a>,
}

impl<'a> Root<'a> {
  pub fn new<S: Into<Cow<'a, str>>>(src: S) -> Self {
    Root {
      source: src.into(),
      document: Document {
        loc: None,
        definitions: Vec::new(),
      },
    }
  }

  pub fn from_file(path: &'a Path) -> Result<Self> {
    use std::fs::File;
    use std::io::Read;

    let mut f = try!(File::open(path.clone()));
    let mut buf = String::new();
    try!(f.read_to_string(&mut buf));
    Ok(Self::new(buf))
  }

  // TODO PARSE
  pub fn parse(mut self) -> Result<Self> {
    {
      use parser::Parser;
      //self.document = try!(Parser::parse(&self.source));
    }
    Ok(self)
  }
}

/// Contains some character offsets that identify where the source of the AST
/// is from. Used as source.body[start..end]
pub struct Location {
  pub start: usize,
  pub end: usize,
}

/// Document : Definition+
pub struct Document<'a> {
  pub loc: Option<Location>,
  pub definitions: Vec<Definition<'a>>,
}

impl_life_node_for! { Document }

/// VariableDefinitions : ( VariableDefinition+ )
pub type VariableDefinitions<'a> = Vec<VariableDefinition<'a>>;

/// VariableDefinition : Variable : Type DefaultValue?
pub struct VariableDefinition<'a> {
  pub loc: Option<Location>,
  pub variable: Variable<'a>,
  pub type_: Type<'a>,
  pub default_value: Option<DefaultValue<'a>>,
}

impl_life_node_for! { VariableDefinition }

/// Directives : Directive+
pub type Directives<'a> = Vec<Directive<'a>>;

/// Directive : @ Name Arguments?
pub struct Directive<'a> {
  pub loc: Option<Location>,
  pub name: Name<'a>,
  pub arguments: Option<Arguments<'a>>,
}

impl_life_node_for! { Directive }

mod definitions;
pub use self::definitions::*;
mod types;
pub use self::types::*;
mod selections;
pub use self::selections::*;
mod values;
pub use self::values::*;
