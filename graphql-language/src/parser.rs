#![allow(dead_code, unused_variables)]
use ast;
use lexer::Punctuator::*;
use lexer::Token::*;
use lexer::{Lexer, Token};
use source::Source;
use std::iter::Peekable;
use std::result;

pub type Result<T> = result::Result<T, &'static str>;

macro_rules! peek {
  ($parser: ident, $($p: pat)|*) => ({
    let mut is_match = false;
    if let &Ok(ref c) = $parser.lexer.peek().unwrap() {
      match c {
        $(
          &$p => {
            is_match = true;
          }
        )*
        _ => {}
      }
    }
    is_match
  });
}

macro_rules! next {
  ($parser: ident) => ({
    let t = $parser.lexer.next().unwrap().unwrap();
    match t {
      Eof => {
        return Err("END OF FILE!");
      },
      Punctuator(_, _, hi) |
      Name(_, _, hi)       |
      IntValue(_, _, hi)   |
      StringValue(_, _, hi)|
      FloatValue(_, _, hi) => {
        $parser.prev = $parser.curr;
        $parser.curr = hi;
      },
    }

    t
  })
}

pub struct Parser<'a> {
  lexer: Peekable<Lexer<'a>>,
  source: &'a Source<'a>,
  prev: usize,
  curr: usize,
}

impl<'a> Parser<'a> {
  pub fn new(src: &'a Source<'a>) -> Self {
    Parser {
      lexer: Lexer::new(src.body).peekable(),
      source: src,
      prev: 0,
      curr: 0,
    }
  }

  fn loc(&self, lo: usize) -> Option<ast::Location<'a>> {
    Some(ast::Location {
      start: lo,
      end: self.curr,
      source: self.source,
    })
  }

  fn parse_definition(&mut self) -> Result<ast::Definition<'a>> {
    match next!(self) {
      Punctuator(LeftBrace, lo, _) => self.parse_short_operation(lo),
      Name(name, lo, _) => {
        match name {
          "query" => self.parse_operation_def(ast::OperationType::Query, name, lo),
          "mutation" => self.parse_operation_def(ast::OperationType::Mutation, name, lo),
          "fragment" => self.parse_fragment_def(name, lo),
          _ => Err("Unkown Op"),
        }
      },
      _ => Err("Unexpected Token"),
    }
  }

  fn parse_short_operation(&mut self, lo: usize) -> Result<ast::Definition<'a>> {
    let selections = try!(self.parse_selection_set(lo));

    Ok(ast::Definition::Operation(
      ast::OperationDefinition {
        loc: self.loc(lo),
        operation: ast::OperationType::Query,
        name: None,
        variable_definitions: None,
        directives: None,
        selection_set: selections,
      }
    ))
  }

  fn parse_field(&mut self, name: &'a str, lo: usize) -> Result<ast::Selection<'a>> {
    let (alias, name) = if peek!(self, Punctuator(Colon, _, _)) {
      let a = try!(self.parse_name(name, lo));
      next!(self);
      (Some(a), match next!(self) {
        Name(n, l, _) => {
          try!(self.parse_name(n, l))
        },
        _ => {
          return Err("Expected Name after colon");
        },
      })
    } else {
      (None, try!(self.parse_name(name, lo)))
    };

    let args = try!(self.parse_arguments(hi));
    let dirs = try!(self.parse_directives(hi));
    let selections = try!(self.parse_selection_set(hi));

    Ok(ast::Selection::Field(ast::Field {
      loc: self.loc(lo),
      alias: alias,
      name: name,
      arguments: args,
      directives: dirs,
      selection_set: Some(selections),
    }))
  }

  fn parse_name(&mut self, name: &'a str, lo: usize) -> Result<ast::Name<'a>> {
    Ok(ast::Name {
      loc: self.loc(lo),
      value: name,
    })
  }

  fn parse_selection_set(&mut self, lo: usize) -> Result<ast::SelectionSet<'a>> {
    let mut hi = lo;
    let mut selections = Vec::new();

    if peek!(self, Punctuator(LeftBrace, _, _)) {
      next!(self); // Required to skip the brace.
      loop {
        match next!(self) {
          Name(name, s, end) => {
            let select = try!(self.parse_field(name, s));
            selections.push(select);
          },
          Punctuator(Spread, lo, _) => {
            let select = try!(self.parse_fragment(lo));
            selections.push(select);
          },
          Punctuator(RightBrace, _, end) => {
            break;
          },
          _ => {
            return Err("Unexpected Token");
          },
        }
      }
    }

    Ok(ast::SelectionSet {
      selections: selections,
      loc: self.loc(lo),
    })
  }

  fn parse_fragment(&mut self, lo: usize) -> Result<ast::Selection<'a>> {
    let mut end = lo;
    //if peek!(self, TokenName("on", _, _)) { FIXME Parse Fragments!
    //  next!(self);
    //
    //} else {
      let name = match next!(self) {
        Name(name, s, hi) => {
          try!(self.parse_name(name, s))
        },
        _ => {
          return Err("Expected Fragment Name");
        }
      };
      let dirs = try!(self.parse_directives(end));
      Ok(ast::Selection::FragmentSpread(ast::FragmentSpread {
        loc: self.loc(lo),
        name: name,
        directives: dirs,
      }))
    //}
  }

  fn parse_arguments(&mut self, lo: usize) -> Result<Option<ast::Arguments<'a>>> {
    let mut args = Vec::new();
    let mut end = lo;

    if peek!(self, Punctuator(LeftParen, _, _)) {
      next!(self);
      loop {
        match next!(self) {
          Name(name, lo, hi) => {
            let name = try!(self.parse_name(name, lo));
            match next!(self) {
              Punctuator(Colon, _, _) => {
                let value = try!(self.parse_value(false));
                args.push(ast::Argument {
                  loc: self.loc(lo),
                  name: name,
                  value: value,
                })
              },
              _ => {
                return Err("Expected Value after colon");
              },
            }
          },
          Punctuator(RightParen, _, _) => {
            break;
          },
          _ => {},
        }
      }
    }

    Ok(some(args))
  }

  fn parse_value(&mut self, is_const: bool) -> Result<ast::Value<'a>> {
    match next!(self) {
      Punctuator(LeftBracket, lo, _) => self.parse_array(is_const, lo),
      Punctuator(LeftBrace, lo, _) => self.parse_object(is_const, lo),
      Punctuator(Dollar, lo, _) => {
        if is_const {
          Err("No value?")
        } else {
          self.parse_variable(lo)
        }
      },
      Name(val, lo, hi) => {
        match val {
          "true" | "false" => {
            Ok(ast::Value::Boolean(ast::BooleanValue {
              loc: self.loc(lo),
              value: val.parse().unwrap(),
            }))
          },
          e if e != "null" => {
            Ok(ast::Value::Enum(ast::EnumValue {
              loc: self.loc(lo),
              name: try!(self.parse_name(val, lo)),
            }))
          },
          _ => Err("Unexpected null"),
        }
      },
      IntValue(val, lo, hi) => {
        Ok(ast::Value::Int(ast::IntValue {
          loc: self.loc(lo),
          value: val,
        }))
      },
      FloatValue(val, lo, hi) => {
        Ok(ast::Value::Float(ast::FloatValue {
          loc: self.loc(lo),
          value: val,
        }))
      },
      StringValue(val, lo, hi) => {
        Ok(ast::Value::String(ast::StringValue {
          loc: self.loc(lo),
          value: val,
        }))
      },
      _ => Err("Unexpected"),
    }
  }

  fn parse_array(&mut self, is_const: bool, lo: usize) -> Result<ast::Value<'a>> {
    unimplemented!()
  }

  fn parse_object(&mut self, is_const: bool, lo: usize) -> Result<ast::Value<'a>> {
    unimplemented!()
  }

  fn parse_variable(&mut self, lo: usize) -> Result<ast::Value<'a>> {
    unimplemented!()
  }

  fn parse_operation_def(&mut self, op: ast::OperationType, name: &'a str, lo: usize) -> Result<ast::Definition<'a>> {
    let name = try!(self.parse_name(name, lo));
    let vars = try!(self.parse_variables(hi));
    let dirs = try!(self.parse_directives(hi));
    let selections = try!(self.parse_selection_set(hi));

    Ok(ast::Definition::Operation(
      ast::OperationDefinition {
        loc: self.loc(lo),
        operation: op,
        name: Some(name),
        variable_definitions: vars,
        directives: dirs,
        selection_set: selections,
      }
    ))
  }

  fn parse_variables(&mut self, lo: usize) -> Result<Option<ast::VariableDefinitions<'a>>> {
    unimplemented!()
  }

  fn parse_directives(&mut self, lo: usize) -> Result<Option<ast::Directives<'a>>> {
    unimplemented!()
  }

  fn parse_fragment_def(&mut self, name: &'a str, lo: usize) -> Result<ast::Definition<'a>> {
    let name = try!(self.parse_name(name, lo));
    let tc = try!(self.parse_type_condition(hi));
    let dirs = try!(self.parse_directives(hi));
    let selections = try!(self.parse_selection_set(hi));

    Ok(ast::Definition::Fragment(
      ast::FragmentDefinition {
        loc: self.loc(lo),
        name: name,
        type_condition: tc,
        directives: dirs,
        selection_set: selections,
      }
    ))
  }

  fn parse_type_condition(&mut self, lo: usize) -> Result<ast::TypeCondition<'a>> {
    unimplemented!()
  }
}

impl<'a> Iterator for Parser<'a> {
  type Item = Result<ast::Definition<'a>>;
  fn next(&mut self) -> Option<Result<ast::Definition<'a>>> {
    if self.lexer.peek().is_some() {
      Some(self.parse_definition())
    } else {
      None
    }
  }
}

fn some<T>(input: Vec<T>) -> Option<Vec<T>> {
  if input.len() > 0 {
    Some(input)
  } else {
    None
  }
}
