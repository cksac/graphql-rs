use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::{Path, PathBuf};

static DEFAULT_SOURCE_NAME: &'static str = "GraphQL";

#[derive(Debug)]
pub struct Source {
  pub name: String,
  pub body: String,
}

impl Source {
  pub fn new(body: String) -> Self {
    Source {
      name: DEFAULT_SOURCE_NAME.to_owned(),
      body: body,
    }
  }

  pub fn with_name(name: String, body: String) -> Self {
    Source {
      name: name,
      body: body,
    }
  }

  pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
    let mut p = PathBuf::new();
    p.push(path);
    let mut f = try!(File::open(p.clone()));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(Source::with_name(Self::get_file_name(p), s))
  }

  fn get_file_name(path: PathBuf) -> String {
    if let Some(n) = path.as_path().file_name() {
      if let Ok(name) = n.to_os_string().into_string() {
        return name;
      }
    }
    DEFAULT_SOURCE_NAME.to_owned()
  }
}
