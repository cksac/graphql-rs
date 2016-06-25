use std::borrow::Cow;
use std::io::Error;
use std::path::Path;

static DEFAULT_SOURCE_NAME: &'static str = "GraphQL";

#[derive(Debug)]
pub struct Source<'a> {
  pub name: Cow<'a, str>,
  pub body: Cow<'a, str>,
}

impl<'a> Source<'a> {
  pub fn new<S: Into<Cow<'a, str>>>(body: S) -> Self {
    Source {
      name: DEFAULT_SOURCE_NAME.into(),
      body: body.into(),
    }
  }

  pub fn name<S: Into<Cow<'a, str>>>(mut self, name: S) -> Self {
    self.name = name.into();
    self
  }

  pub fn from_file(path: &'a Path) -> Result<Source<'a>, Error> {
    use std::fs::File;
    use std::io::Read;
    use std::ffi::OsStr;

    let mut f = try!(File::open(path.clone()));
    let mut buf = String::new();
    try!(f.read_to_string(&mut buf));
    Ok(Source::new(buf).name(path.file_name()
                                 .and_then(OsStr::to_str)
                                 .unwrap_or(DEFAULT_SOURCE_NAME)))
  }
}
