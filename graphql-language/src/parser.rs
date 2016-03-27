#![allow(dead_code, unused_variables)]
use ast::*;
use lexer::Punctuator::*;
use lexer::Token::{
  Eof,
  Punctuator,
  Name as TokenName,
  IntValue as TokenInt,
  FloatValue as TokenFloat,
  StringValue as TokenString,
};
use lexer::{Lexer, Token};
use source::Source;
use std::iter::Peekable;

pub type Res<'a, T> = Result<T, &'a str>;
pub type DRes<'a, T> = Res<'a, (T, usize)>;

pub struct Parser<'a> {
  lexer: Peekable<Lexer<'a>>,
  source: Source<'a>,
}

impl<'a> Parser<'a> {
  pub fn new(src: Source<'a>) -> Self {
    Parser {
      lexer: Lexer::new(src.body).peekable(),
      source: src,
    }
  }

  fn loc(&self, lo: usize, hi: usize) -> Location<'a> {
    Location {
      start: lo,
      end: hi,
      source: self.source,
    }
  }
}

impl<'a> Iterator for Parser<'a> {
  type Item = Res<'a, Definition<'a>>;
  fn next(&mut self) -> Option<Res<'a, Definition<'a>>> {
    if self.lexer.peek().is_some() {
      Some(parse_definition(self))
    } else {
      None
    }
  }
}

macro_rules! on_do {
 ($parser: ident, $($p: pat => $b: block)+) => {
   loop {
     match next!($parser) {
       $(
         $p => $b
       )+
     }
   }
 }
}

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
  ($parser: ident) => {$parser.lexer.next().unwrap().unwrap()}
}

fn parse_definition<'a>(parser: &mut Parser<'a>) -> Res<'a, Definition<'a>> {
  let token = next!(parser);

  match token {
    Punctuator(LeftBrace, lo, _) => parse_short_operation(parser, lo),
    TokenName(name, lo, hi) => {
      match name {
        "query" => parse_operation_def(parser, OperationType::Query, name, lo, hi),
        "mutation" => parse_operation_def(parser, OperationType::Mutation, name, lo, hi),
        "fragment" => parse_fragment_def(parser, name, lo, hi),
        _ => Err("Unkown Op"),
      }
    },
    _ => Err("Unexpected Token"),
  }
}

fn parse_short_operation<'a>(parser: &mut Parser<'a>, lo: usize) -> Res<'a, Definition<'a>> {
  let (selections, hi) = try!(parse_selection_set(parser, lo));

  Ok(Definition::Operation(
    OperationDefinition {
      loc: parser.loc(lo, hi),
      operation: OperationType::Query,
      name: None,
      variable_definitions: None,
      directives: None,
      selection_set: selections,
    }
  ))
}

fn parse_field<'a>(parser: &mut Parser<'a>, name: &'a str, lo: usize, mut hi: usize) -> DRes<'a, Selection<'a>> {
  let (alias, name) = if peek!(parser, Punctuator(Colon, _, _)) {
      let a = try!(parse_name(parser, name, lo, hi));
      next!(parser);
      (Some(a), match next!(parser) {
        TokenName(n, l, h) => {
          hi = h;
          try!(parse_name(parser, n, l, h))
        },
        _ => {
          return Err("Expected Name after colon");
        },
      })
  } else {
    (None, try!(parse_name(parser, name, lo, hi)))
  };

  let (args, hi) = try!(parse_arguments(parser, hi));
  let (dirs, hi) = try!(parse_directives(parser, hi));
  let (selections, hi) = try!(parse_selection_set(parser, hi));

  Ok((Selection::Field(Field {
    loc: parser.loc(lo, hi),
    alias: alias,
    name: name,
    arguments: args,
    directives: dirs,
    selection_set: Some(selections),
  }), hi))
}

fn parse_name<'a>(parser: &mut Parser<'a>, name: &'a str, lo: usize, hi: usize) -> Res<'a, Name<'a>> {
  Ok(Name {
    loc: parser.loc(lo, hi),
    value: name,
  })
}

fn parse_selection_set<'a>(parser: &mut Parser<'a>, lo: usize) -> DRes<'a, SelectionSet<'a>> {
  let mut hi = lo;
  let mut selections = Vec::new();

  if peek!(parser, Punctuator(LeftBrace, _, _)) {
    next!(parser);
    on_do! {
      parser,
      TokenName(name, s, end) => {
        let (select, e) = try!(parse_field(parser, name, s, end));
        hi = e;
        selections.push(select);
      }
      Punctuator(Spread, lo, _) => {
        let (select, e) = try!(parse_fragment(parser, lo));
        hi = e;
        selections.push(select);
      }
      Punctuator(RightBrace, _, end) => {
        hi = end;
        break;
      }
      _ => {
        return Err("Unexpected Token");
      }
    }
  }

  Ok((SelectionSet {
    selections: selections,
    loc: parser.loc(lo, hi),
  }, hi))
}

fn parse_fragment<'a>(parser: &mut Parser<'a>, lo: usize) -> DRes<'a, Selection<'a>> {
  let mut end = lo;
  //if peek!(parser, TokenName("on", _, _)) { FIXME Parse Fragments!
  //  next!(parser);
  //
  //} else {
    let name = match next!(parser) {
      TokenName(name, s, hi) => {
        end = hi;
        try!(parse_name(parser, name, s, hi))
      },
      _ => {
        return Err("Expected Fragment Name");
      }
    };
    let (dirs, hi) = parse_directives(parser, end);
    Selection::FragmentSpread(FragmentSpread {
      loc: parser.loc(lo, hi),
      name: name,
      directives: dirs,
    })
  //}
}

fn parse_arguments<'a>(parser: &mut Parser<'a>, lo: usize) -> DRes<'a, Option<Arguments<'a>>> {
  let mut args = Vec::new();
  let mut end = lo;

  if peek!(parser, Punctuator(LeftParen, _, _)) {
    next!(parser);
    on_do!{
      parser,
      TokenName(name, lo, hi) => {
        let name = try!(parse_name(parser, name, lo, hi));
        match next!(parser) {
          Punctuator(Colon, _, _) => {
            let (value, hi) = try!(parse_value(parser, false));
            args.push(Argument {
              loc: parser.loc(lo, hi),
              name: name,
              value: value,
            })
          },
          _ => {
            return Err("Expected Value after colon");
          },
        }
      }
      Punctuator(RightParen, _, hi) => {
        end = hi;
        break;
      }
      _ => {}
    }
  }

  Ok((some(args), end))
}

fn parse_value<'a>(parser: &mut Parser<'a>, is_const: bool) -> DRes<'a, Value<'a>> {
  match next!(parser) {
    Punctuator(LeftBracket, lo, _) => parse_array(parser, is_const, lo),
    Punctuator(LeftBrace, lo, _) => parse_object(parser, is_const, lo),
    Punctuator(Dollar, lo, _) => {
      if is_const {
        return Err("No value?");
      } else {
        parse_variable(parser, lo)
      }
    },
    TokenName(val, lo, hi) => {
      match val {
        "true" | "false" => {
          Value::Boolean(BooleanValue {
            loc: parser.loc(lo, hi),
            value: val.parse().unwrap(),
          })
        },
        e if e != "null" => {
          Value::Enum(EnumValue {
            loc: parser.loc(lo, hi),
            name: parse_name(val, lo, hi).unwrap(),
          })
        },
        _ => {
          return Err("Unexpected name");
        }
      }
    },
    TokenInt(val, lo, hi) => {
      Value::Int(IntValue {
        loc: parser.loc(lo, hi),
        value: val,
      })
    },
    TokenFloat(val, lo, hi) => {
      Value::Float(FloatValue {
        loc: parser.loc(lo, hi),
        value: val,
      })
    },
    TokenString(val, lo, hi) => {
      Value::String(StringValue {
        loc: parser.loc(lo, hi),
        value: val,
      })
    },
    _ => {
      return Err("Unexpected");
    }
  }
}

fn parse_array<'a>(parser: &mut Parser<'a>, is_const: bool, lo: usize) -> DRes<'a, Value<'a>> {
  unimplemented!()
}

fn parse_object<'a>(parser: &mut Parser<'a>, is_const: bool, lo: usize) -> DRes<'a, Value<'a>> {
  unimplemented!()
}

fn parse_operation_def<'a>(parser: &mut Parser<'a>, op: OperationType, name: &'a str, lo: usize, hi: usize) -> Res<'a, Definition<'a>> {
  let name = try!(parse_name(parser, name, lo, hi));
  let (vars, hi) = try!(parse_variables(parser, hi));
  let (dirs, hi) = try!(parse_directives(parser, hi));
  let (selections, hi) = try!(parse_selection_set(parser, hi));

  Ok(Definition::Operation(
    OperationDefinition {
      loc: parser.loc(lo, hi),
      operation: op,
      name: Some(name),
      variable_definitions: vars,
      directives: dirs,
      selection_set: selections,
    }
  ))
}

fn parse_variables<'a>(parser: &mut Parser<'a>, lo: usize) -> DRes<'a, Option<VariableDefinitions<'a>>> {
  unimplemented!()
}

fn parse_directives<'a>(parser: &mut Parser<'a>, lo: usize) -> DRes<'a, Option<Directives<'a>>> {
  unimplemented!()
}

fn parse_fragment_def<'a>(parser: &mut Parser<'a>, name: &'a str, lo: usize, hi: usize) -> Res<'a, Definition<'a>> {
  let name = try!(parse_name(parser, name, lo, hi));
  let (tc, hi) = try!(parse_type_condition(parser, hi));
  let (dirs, hi) = try!(parse_directives(parser, hi));
  let (selections, hi) = try!(parse_selection_set(parser, hi));

  Ok(Definition::Fragment(
    FragmentDefinition {
      loc: parser.loc(lo, hi),
      name: name,
      type_condition: tc,
      directives: dirs,
      selection_set: selections,
    }
  ))
}

fn parse_type_condition<'a>(parser: &mut Parser<'a>, lo: usize) -> DRes<'a, TypeCondition<'a>> {
  unimplemented!()
}

fn some<T>(input: Vec<T>) -> Option<Vec<T>> {
  if input.len() > 0 {
    Some(input)
  } else {
    None
  }
}
