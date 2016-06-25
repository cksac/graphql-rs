mod error;
pub use self::error::Error;
use std::result;
pub type Result<T> = result::Result<T, Error>;

use ast;
use lexer::Punctuator::*;
use lexer::Token::*;
use lexer::Lexer;
use std::iter::Peekable;

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

// I need a better name for this,
// but I will not write `self.lexer.peek().unwrap()` *every* time I call this!
macro_rules! peek_next {
  ($parser: ident) => ({
    if let &Ok(ref c) = $parser.lexer.peek().unwrap() {
      c
    } else {
      // TODO Remember what this is and make it not panic?
      panic!("ERROR!")
    }
  })
}

macro_rules! next {
  ($parser: ident) => ({
    let t = $parser.lexer.next().unwrap().unwrap();
    match t {
      Eof => {
        return Err(Error::Eof);
      },
      Punctuator (_, _, hi) |
      Name       (_, _, hi) |
      IntValue   (_, _, hi) |
      StringValue(_, _, hi) |
      FloatValue (_, _, hi) => {
        $parser.prev = $parser.curr;
        $parser.curr = hi;
      },
    }

    t
  })
}

macro_rules! value {
  ($parser: ident) => ({
    match next!($parser) {
      Eof | Punctuator(_,_,_) => {
        // TODO Find which error fits this better
        return Err(Error::ExpectedValueNotFound);//Err("NO VALUE!");
      },
      Name       (v, _, _) |
      IntValue   (v, _, _) |
      StringValue(v, _, _) |
      FloatValue (v, _, _) => {
        v
      },
    }
  })
}

pub struct Parser<'a> {
  lexer: Peekable<Lexer<'a>>,
  prev: usize,
  curr: usize,
}

impl<'a> Parser<'a> {
  pub fn parse(src: &'a str) -> Result<ast::Document<'a>> {
      let parser = Parser {
        lexer: Lexer::new(src).peekable(),
        prev: 0,
        curr: 0,
      };

      unimplemented!()
  }

  fn loc(&self) -> ast::Location {
    ast::Location {
      start: self.prev,
      end: self.curr,
    }
  }

  // Parser Bits

// DONE
  fn parse_definition(&mut self) -> Result<ast::Definition<'a>> {
    match next!(self) {
      Punctuator(LeftBrace, _, _) => self.parse_short_operation(),
      Name(name, _, _) => {
        match name {
          "query" => self.parse_operation_def(ast::OperationType::Query),
          "mutation" => self.parse_operation_def(ast::OperationType::Mutation),
          "fragment" => self.parse_fragment_def(),
          _ => Err(Error::UnkownOperation),
        }
      }
      _ => Err(Error::UnexpectedToken),
    }
  }

// DONE
  fn parse_short_operation(&mut self) -> Result<ast::Definition<'a>> {
    let mut loc = self.loc();
    let selections = try!(self.parse_selection_set());
    loc.end = self.curr;

    Ok(ast::Definition::Operation(ast::OperationDefinition {
      loc: Some(loc),
      operation: ast::OperationType::Query,
      name: None,
      variable_definitions: None,
      directives: None,
      selection_set: selections,
    }))
  }

// DONE
  fn parse_selection_set(&mut self) -> Result<ast::SelectionSet<'a>> {
    if peek!(self, Punctuator(LeftBrace, _, _)) {
      let mut loc = self.loc();
      let mut selections = Vec::new();
      next!(self); // Required to skip the start brace.

      loop {
        match peek_next!(self) {
          &Name(_, _, _) => selections.push(try!(self.parse_field())),
          &Punctuator(Spread, _, _) => selections.push(try!(self.parse_fragment())),
          &Punctuator(RightBrace, _, _) => {
            next!(self); // Required to skip the end brace.
            break;
          }
          _ => {
            return Err(Error::UnexpectedToken);
          }
        }
      }
      loc.end = self.curr;

      Ok(ast::SelectionSet {
        selections: selections,
        loc: Some(loc),
      })
    } else {
      Err(Error::MissingExpectedToken)//"Expected Left Brace not found!")
    }
  }

// DONE
  fn parse_field(&mut self) -> Result<ast::Selection<'a>> {
    let mut loc = self.loc();

    let (alias, name) = {
      let ret = try!(self.parse_name());

      if peek!(self, Punctuator(Colon, _, _)) {
        next!(self); // Skip colon.
        (Some(ret),
         if peek!(self, Name(_, _, _)) {
          try!(self.parse_name())
        } else {
          return Err(Error::MissingExpectedToken);//"Expected Name after colon"); // BAIL OUT!
        })
      } else {
        (None, ret)
      }
    };

    let args = self.parse_arguments().ok();
    let dirs = self.parse_directives().ok();
    let selections = self.parse_selection_set().ok();
    loc.end = self.curr;

    Ok(ast::Selection::Field(ast::Field {
      loc: Some(loc),
      alias: alias,
      name: name,
      arguments: args,
      directives: dirs,
      selection_set: selections,
    }))
  }

// DONE
  fn parse_name(&mut self) -> Result<ast::Name<'a>> {
    if peek!(self, Name(_, _, _)) {
      let v = value!(self);
      Ok(ast::Name {
        loc: Some(self.loc()),
        value: v,
      })
    } else {
      Err(Error::MissingExpectedToken)//"Expected a name!")
    }
  }

// DONE
  fn parse_arguments(&mut self) -> Result<ast::Arguments<'a>> {
    if peek!(self, Punctuator(LeftParen, _, _)) {
      let mut args = Vec::new();
      next!(self); // Required to skip the start paren.

      loop {
        match peek_next!(self) {
          &Name(_, _, _) => {
            let mut loc = self.loc();
            let name = try!(self.parse_name());
            match next!(self) {
              Punctuator(Colon, _, _) => {
                let value = try!(self.parse_value(false));
                loc.end = self.curr;
                args.push(ast::Argument {
                  loc: Some(loc),
                  name: name,
                  value: value,
                })
              }
              _ => {
                return Err(Error::MissingExpectedToken);//"Expected Value after colon");
              }
            }
          }
          &Punctuator(RightParen, _, _) => {
            break;
          }
          _ => {
            return Err(Error::UnexpectedToken);
          }
        }
      }

      Ok(args)
    } else {
      Err(Error::MissingExpectedToken)//"Expected Left Paren not found!")
    }
  }

  fn parse_directives(&mut self) -> Result<ast::Directives<'a>> {
    unimplemented!()
  }

  // FIXME Parse Fragments!
  fn parse_fragment(&mut self) -> Result<ast::Selection<'a>> {
    let mut loc = self.loc();

    // if peek!(self, TokenName("on", _, _)) {
    //  next!(self);
    //
    // } else {
    let name = try!(self.parse_name());
    let dirs = self.parse_directives().ok();
    loc.end = self.curr;

    Ok(ast::Selection::FragmentSpread(ast::FragmentSpread {
      loc: Some(loc),
      name: name,
      directives: dirs,
    }))
    // }
  }

// DONE
  fn parse_operation_def(&mut self, op: ast::OperationType) -> Result<ast::Definition<'a>> {
    let mut loc = self.loc();
    let name = self.parse_name().ok();
    let vars = self.parse_variables().ok();
    let dirs = self.parse_directives().ok();
    let selections = try!(self.parse_selection_set());
    loc.end = self.curr;

    Ok(ast::Definition::Operation(ast::OperationDefinition {
      loc: Some(loc),
      operation: op,
      name: name,
      variable_definitions: vars,
      directives: dirs,
      selection_set: selections,
    }))
  }

  fn parse_variables(&mut self) -> Result<ast::VariableDefinitions<'a>> {
    unimplemented!()
  }

// DONE
  fn parse_value(&mut self, is_const: bool) -> Result<ast::Value<'a>> {
    match peek_next!(self) {
      &Punctuator(LeftBracket, _, _) => self.parse_array(is_const),
      &Punctuator(LeftBrace, _, _) => self.parse_object(is_const),
      &Punctuator(Dollar, _, _) => {
        if is_const {
          Err(Error::ExpectedValueNotFound)
        } else {
          self.parse_variable()
        }
      }
      &Name(_, _, _) => {
        let name = try!(self.parse_name());
        match name.value {
          "true" | "false" => {
            Ok(ast::Value::Boolean(ast::BooleanValue {
              loc: name.loc,
              value: name.value.parse().unwrap(),
            }))
          }
          e if e != "null" => {
            Ok(ast::Value::Enum(name))
          }
          _ => Err(Error::ExpectedValueNotFound),//"Unexpected null"),
        }
      }
      &IntValue(_, _, _) => {
        let val = value!(self).parse().unwrap();
        Ok(ast::Value::Int(ast::IntValue {
          loc: Some(self.loc()),
          value: val,
        }))
      }
      &FloatValue(_, _, _) => {
        let val = value!(self).parse().unwrap();
        Ok(ast::Value::Float(ast::FloatValue {
          loc: Some(self.loc()),
          value: val,
        }))
      }
      &StringValue(_, _, _) => {
        let val = value!(self).to_owned();
        Ok(ast::Value::String(ast::StringValue {
          loc: Some(self.loc()),
          value: val,
        }))
      }
      _ => Err(Error::UnexpectedToken),//"Unexpected"),
    }
  }

  fn parse_array(&mut self, is_const: bool) -> Result<ast::Value<'a>> {
    unimplemented!()
  }

  fn parse_object(&mut self, is_const: bool) -> Result<ast::Value<'a>> {
    unimplemented!()
  }

  fn parse_variable(&mut self) -> Result<ast::Value<'a>> {
    unimplemented!()
  }

// DONE
  fn parse_fragment_def(&mut self) -> Result<ast::Definition<'a>> {
    let mut loc = self.loc();
    let name = try!(self.parse_name());
    let tc = try!(self.parse_type_condition());
    let dirs = self.parse_directives().ok();
    let selections = try!(self.parse_selection_set());
    loc.end = self.curr;

    Ok(ast::Definition::Fragment(ast::FragmentDefinition {
      loc: Some(loc),
      name: name,
      type_condition: tc,
      directives: dirs,
      selection_set: selections,
    }))
  }

  fn parse_type_condition(&mut self) -> Result<ast::TypeCondition<'a>> {
    unimplemented!()
  }
}
