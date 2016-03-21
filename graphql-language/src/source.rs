static DEFAULT_SOURCE_NAME: &'static str = "GraphQL";

#[derive(Debug)]
pub struct Source<'a> {
  pub name: &'a str,
  pub body: &'a str,
}

impl<'a> Source<'a> {
  pub fn new(body: &'a str) -> Self {
    Source {
      name: DEFAULT_SOURCE_NAME,
      body: body,
    }
  }

  pub fn name(mut self, name: &'a str) -> Self {
    self.name = name;
    self
  }
}

use std::io::Error;
use std::path::Path;

pub fn from_file<'a>(path: &'a Path, buf: &'a mut String) -> Result<Source<'a>, Error> {
  use std::fs::File;
  use std::io::Read;
  use std::ffi::OsStr;

  let mut f = try!(File::open(path.clone()));
  try!(f.read_to_string(buf));
  Ok(Source::new(buf.as_str()).name(path
    .file_name()
    .and_then(OsStr::to_str)
    .unwrap_or(DEFAULT_SOURCE_NAME)))
}
