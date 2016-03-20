static DEFAULT_SOURCE_NAME: &'static str = "GraphQL";

#[derive(Debug)]
pub struct Source<'a> {
  pub name: &'a str,
  pub body: &'a str,
}

impl<'a> Source<'a> {
  pub fn new(body: &'a str) -> Source<'a> {
    Source {
      name: DEFAULT_SOURCE_NAME,
      body: body,
    }
  }

  pub fn with_name(name: &'a str, body: &'a str) -> Source<'a> {
    Source {
      name: name,
      body: body,
    }
  }
}