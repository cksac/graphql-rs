use std::collections::HashMap;
use std::str::FromStr;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

macro_rules! blanket_impl {
  ($trait_: ident for $($type_: ty),*) => {
    $(
      impl $trait_ for $type_ {}
    )*
  };
}

macro_rules! impl_graphql_type_for {
    ($tt: ty where name = $name: expr, description = $desc: expr) => {
        impl GraphQLType for $tt {
            fn name(&self) -> &str { $name }
            fn description(&self) -> Option<&str> { Some($desc) }
        }
    };

    ($($type_: ty),*) => {
      $(
        impl GraphQLType for $type_ {
            fn name(&self) -> &str { self.name.as_ref() }

            fn description(&self) -> Option<&str> {
                self.description.as_ref().map(|s| s.as_ref())
            }
        }
      )*
    };    
}

macro_rules! impl_scalar_type_for {
    ($tt: ty as $value_type: ident where name = $name: expr, description = $desc: expr) => {
      impl_graphql_type_for! { $tt where name = $name, description = $desc }

      impl GraphQLScalar for $tt {
        type ValueType = $value_type;
        fn coerce_literal(&self, value: &str) -> Option<Self::ValueType> {
          $value_type::from_str(value).ok()
        }
      }
    };
}

pub trait GraphQLType {
  fn name(&self) -> &str;
  fn description(&self) -> Option<&str>;
}
impl_graphql_type_for! { GraphQLEnum, GraphQLObject, GraphQLUnion, GraphQLInterface }

pub trait GraphQLScalar: GraphQLType {
  type ValueType: Any;
  fn coerce_literal(&self, value: &str) -> Option<Self::ValueType>;
}

pub struct GraphQLInt;
impl_scalar_type_for! { GraphQLInt as i32 where
  name = "Int",
  description = "The Int scalar type represents a signed 32‐bit numeric non‐fractional values."
}

pub struct GraphQLFloat;
impl_scalar_type_for! { GraphQLFloat as f64 where
  name = "Float",
  description = "The Float scalar type represents signed double-precision fractional values as specified by IEEE 754."
}

pub struct GraphQLString;
impl_scalar_type_for! { GraphQLString as String where
  name = "String",
  description = "The String scalar type represents textual data, represented as UTF-8 character sequences."
}

pub struct GraphQLBoolean;
impl_scalar_type_for! { GraphQLBoolean as bool where
  name = "Boolean",
  description = "The Boolean scalar type represents true or false."
}

pub struct GraphQLEnum {
  name: String,
  description: Option<String>,
  values: HashMap<String, GraphQLEnumValue>,
}

pub struct GraphQLEnumValue {
  value: String,
  description: Option<String>,
  depreciation_reason: Option<String>,
}

pub trait GraphQLOutput: GraphQLType {}
blanket_impl! { GraphQLOutput for GraphQLInt, GraphQLFloat, GraphQLString, GraphQLBoolean, GraphQLObject, GraphQLInterface }
impl<T> GraphQLOutput for GraphQLScalar<ValueType = T> {}

pub trait GraphQLInput: GraphQLType {}
blanket_impl! { GraphQLInput for GraphQLInt, GraphQLFloat, GraphQLString, GraphQLBoolean }
impl<T> GraphQLInput for GraphQLScalar<ValueType = T> {}

pub struct GraphQLObject {
  name: String,
  description: Option<String>,
  fields: RefCell<HashMap<String, GraphQLField>>,
  interfaces: Option<HashMap<String, Rc<GraphQLInterface>>>,
}

impl GraphQLObject {
  pub fn replace_field_placeholder_type<T: GraphQLOutput + 'static>(&self,
                                                                    field_name: &str,
                                                                    other_type: &Rc<T>) {
    let mut field = self.fields.borrow_mut().remove(field_name);
    if field.is_none() {
      panic!("Object type {:} does not have placeholder {:} field.",
             self.name,
             field_name);
    }

    if let Some(mut f) = field {
      let f_type_name = f.typ.name().to_owned();
      if !f_type_name.ends_with("___TypePlaceholder___") {
        panic!("Field {:} in object type {:} is not a placeholder.",
               field_name,
               self.name);
      }

      let target_type = f_type_name.trim_right_matches("___TypePlaceholder___");
      if target_type != other_type.name() {
        panic!("Placeholder {:} in object type {:} should replaced by {:} type instead of \
                {:} type.",
               field_name,
               self.name,
               target_type,
               other_type.name());
      }

      f.typ = other_type.clone();
      self.fields.borrow_mut().insert(field_name.to_owned(), f);
    }
  }
}

pub struct GraphQLField {
  name: String,
  description: Option<String>,
  depreciation_reason: Option<String>,
  typ: Rc<GraphQLOutput>,
  args: Option<HashMap<String, GraphQLArgument>>,
}

pub struct GraphQLArgument {
  name: String,
  description: Option<String>,
  typ: Rc<GraphQLInput>,
}

pub struct GraphQLUnion {
  name: String,
  description: Option<String>,
  types: HashMap<String, Rc<GraphQLObject>>,
}

pub struct GraphQLInterface {
  name: String,
  description: Option<String>,
  fields: RefCell<HashMap<String, GraphQLField>>,
}

impl GraphQLInterface {
  pub fn replace_field_placeholder_type<T: GraphQLOutput + 'static>(&self,
                                                                    field_name: &str,
                                                                    other_type: &Rc<T>) {
    let mut field = self.fields.borrow_mut().remove(field_name);
    if field.is_none() {
      panic!("Object type {:} does not have placeholder {:} field.",
             self.name,
             field_name);
    }

    if let Some(mut f) = field {
      let f_type_name = f.typ.name().to_owned();
      if !f_type_name.ends_with("___TypePlaceholder___") {
        panic!("Field {:} in object type {:} is not a placeholder.",
               field_name,
               self.name);
      }

      let target_type = f_type_name.trim_right_matches("___TypePlaceholder___");
      if target_type != other_type.name() {
        panic!("Placeholder {:} in object type {:} should replaced by {:} type instead of \
                {:} type.",
               field_name,
               self.name,
               target_type,
               other_type.name());
      }

      f.typ = other_type.clone();
      self.fields.borrow_mut().insert(field_name.to_owned(), f);
    }
  }
}


// Builders
struct Placeholder {
  name: String,
}

impl Placeholder {
  fn new(target_type_name: &str) -> Placeholder {
    Placeholder { name: format!("{:}___TypePlaceholder___", target_type_name) }
  }
}

impl GraphQLType for Placeholder {
  fn name(&self) -> &str {
    self.name.as_ref()
  }

  fn description(&self) -> Option<&str> {
    None
  }
}

blanket_impl! { GraphQLOutput for Placeholder }

pub struct GraphQLObjectBuilder {
  name: String,
  description: Option<String>,
  fields: HashMap<String, GraphQLField>,
  interfaces: Option<HashMap<String, Rc<GraphQLInterface>>>,
}

impl GraphQLObjectBuilder {
  pub fn new(name: &str) -> GraphQLObjectBuilder {
    GraphQLObjectBuilder {
      name: name.to_owned(),
      description: None,
      fields: HashMap::new(),
      interfaces: None,
    }
  }

  pub fn with_description(mut self, description: &str) -> GraphQLObjectBuilder {
    self.description = Some(description.to_owned());
    self
  }

  pub fn field<F>(mut self, name: &str, f: F) -> GraphQLObjectBuilder
    where F: Fn(GraphQLFieldBuilder) -> GraphQLFieldBuilder
  {
    let field = f(GraphQLFieldBuilder::new(name)).build();
    self.fields.insert(name.to_owned(), field);
    self
  }

  pub fn impl_interface(mut self, interface: &Rc<GraphQLInterface>) -> GraphQLObjectBuilder {
    match self.interfaces {
      Some(ref mut interfaces) => {
        interfaces.insert(interface.name().to_owned(), interface.clone());
      }
      None => {
        let mut interfaces = HashMap::new();
        interfaces.insert(interface.name().to_owned(), interface.clone());
        self.interfaces = Some(interfaces);
      }
    }
    self
  }

  pub fn build(self) -> Rc<GraphQLObject> {
    if self.fields.len() == 0 {
      panic!("Object type {:} must contains at least one field",
             self.name);
    }

    Rc::new(GraphQLObject {
      name: self.name,
      description: self.description,
      fields: RefCell::new(self.fields),
      interfaces: self.interfaces,
    })
  }
}

pub struct GraphQLFieldBuilder {
  name: String,
  description: Option<String>,
  depreciation_reason: Option<String>,
  typ: Option<Rc<GraphQLOutput>>,
  args: Option<HashMap<String, GraphQLArgument>>,
}

impl GraphQLFieldBuilder {
  fn new(name: &str) -> GraphQLFieldBuilder {
    GraphQLFieldBuilder {
      name: name.to_owned(),
      description: None,
      depreciation_reason: None,
      typ: None,
      args: None,
    }
  }

  pub fn with_description(mut self, description: &str) -> GraphQLFieldBuilder {
    self.description = Some(description.to_owned());
    self
  }

  pub fn mark_depreciated(mut self, reason: &str) -> GraphQLFieldBuilder {
    self.depreciation_reason = Some(reason.to_owned());
    self
  }

  pub fn type_of<T: GraphQLOutput + 'static>(mut self, typ: &Rc<T>) -> GraphQLFieldBuilder {
    self.typ = Some(typ.clone());
    self
  }

  pub fn placeholder_type_of(mut self, target_type: &str) -> GraphQLFieldBuilder {
    self.typ = Some(Rc::new(Placeholder::new(target_type)));
    self
  }

  pub fn arg<F>(mut self, name: &str, f: F) -> GraphQLFieldBuilder
    where F: Fn(GraphQLArgumentBuilder) -> GraphQLArgumentBuilder
  {
    let arg = f(GraphQLArgumentBuilder::new(name)).build();
    match self.args {
      Some(ref mut args) => {
        args.insert(name.to_owned(), arg);
      }
      None => {
        let mut args = HashMap::new();
        args.insert(name.to_owned(), arg);
        self.args = Some(args);
      }
    }
    self
  }

  pub fn build(self) -> GraphQLField {
    if self.typ.is_none() {
      panic!("Field {:} missing type defination", self.name);
    }

    GraphQLField {
      name: self.name,
      description: self.description,
      depreciation_reason: self.depreciation_reason,
      typ: self.typ.unwrap(),
      args: None,
    }
  }
}

pub struct GraphQLArgumentBuilder {
  name: String,
  description: Option<String>,
  typ: Option<Rc<GraphQLInput>>,
}

impl GraphQLArgumentBuilder {
  pub fn new(name: &str) -> GraphQLArgumentBuilder {
    GraphQLArgumentBuilder {
      name: name.to_owned(),
      description: None,
      typ: None,
    }
  }

  pub fn type_of<T: GraphQLInput + 'static>(mut self, typ: &Rc<T>) -> GraphQLArgumentBuilder {
    self.typ = Some(typ.clone());
    self
  }

  pub fn build(self) -> GraphQLArgument {
    if self.typ.is_none() {
      panic!("Argument {:} missing type defination", self.name);
    }

    GraphQLArgument {
      name: self.name,
      description: self.description,
      typ: self.typ.unwrap(),
    }
  }
}

pub struct GraphQLEnumBuilder {
  name: String,
  description: Option<String>,
  values: HashMap<String, GraphQLEnumValue>,
}

impl GraphQLEnumBuilder {
  pub fn new(name: &str) -> GraphQLEnumBuilder {
    GraphQLEnumBuilder {
      name: name.to_owned(),
      description: None,
      values: HashMap::new(),
    }
  }

  pub fn with_description(mut self, description: &str) -> GraphQLEnumBuilder {
    self.description = Some(description.to_owned());
    self
  }

  pub fn value<F>(mut self, name: &str, f: F) -> GraphQLEnumBuilder
    where F: Fn(GraphQLEnumValueBuilder) -> GraphQLEnumValueBuilder
  {
    let v = f(GraphQLEnumValueBuilder::new(name)).build();
    self.values.insert(name.to_owned(), v);
    self
  }

  pub fn build(self) -> GraphQLEnum {
    if self.values.len() == 0 {
      panic!("Enum {:} must has at least one value defined.", self.name);
    }

    GraphQLEnum {
      name: self.name,
      description: self.description,
      values: self.values,
    }
  }
}

pub struct GraphQLEnumValueBuilder {
  value: String,
  description: Option<String>,
  depreciation_reason: Option<String>,
}

impl GraphQLEnumValueBuilder {
  pub fn new(value: &str) -> GraphQLEnumValueBuilder {
    GraphQLEnumValueBuilder {
      value: value.to_owned(),
      description: None,
      depreciation_reason: None,
    }
  }

  pub fn with_description(mut self, description: &str) -> GraphQLEnumValueBuilder {
    self.description = Some(description.to_owned());
    self
  }

  pub fn mark_depreciated(mut self, depreciation_reason: &str) -> GraphQLEnumValueBuilder {
    let reason = depreciation_reason.trim().to_owned();
    if reason.len() == 0 {
      panic!("Deprecation reason for enum value {:} cannot be empty",
             self.value);
    }
    self.depreciation_reason = Some(reason);
    self
  }

  pub fn build(self) -> GraphQLEnumValue {
    GraphQLEnumValue {
      value: self.value,
      description: self.description,
      depreciation_reason: self.depreciation_reason,
    }
  }
}

pub struct GraphQLUnionBuilder {
  name: String,
  description: Option<String>,
  types: HashMap<String, Rc<GraphQLObject>>,
}

impl GraphQLUnionBuilder {
  fn new(name: &str) -> GraphQLUnionBuilder {
    GraphQLUnionBuilder {
      name: name.to_owned(),
      description: None,
      types: HashMap::new(),
    }
  }

  pub fn with_description(mut self, description: &str) -> GraphQLUnionBuilder {
    self.description = Some(description.to_owned());
    self
  }

  pub fn maybe_type_of(mut self, typ: &Rc<GraphQLObject>) -> GraphQLUnionBuilder {
    self.types.insert(typ.name().to_owned(), typ.clone());
    self
  }

  pub fn build(self) -> Rc<GraphQLUnion> {
    if self.types.len() == 0 {
      panic!("Union {:} must has at least one possible type defined.",
             self.name);
    }

    Rc::new(GraphQLUnion {
      name: self.name,
      description: self.description,
      types: self.types,
    })
  }
}

pub struct GraphQLInterfaceBuilder {
  name: String,
  description: Option<String>,
  fields: HashMap<String, GraphQLField>,
}

impl GraphQLInterfaceBuilder {
  pub fn new(name: &str) -> GraphQLInterfaceBuilder {
    GraphQLInterfaceBuilder {
      name: name.to_owned(),
      description: None,
      fields: HashMap::new(),
    }
  }

  pub fn with_description(mut self, description: &str) -> GraphQLInterfaceBuilder {
    self.description = Some(description.to_owned());
    self
  }

  pub fn field<F>(mut self, name: &str, f: F) -> GraphQLInterfaceBuilder
    where F: Fn(GraphQLFieldBuilder) -> GraphQLFieldBuilder
  {
    let field = f(GraphQLFieldBuilder::new(name)).build();
    self.fields.insert(name.to_owned(), field);
    self
  }

  pub fn build(self) -> Rc<GraphQLInterface> {
    if self.fields.len() == 0 {
      panic!("Object type {:} must contains at least one field",
             self.name);
    }

    Rc::new(GraphQLInterface {
      name: self.name,
      description: self.description,
      fields: RefCell::new(self.fields),
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::rc::Rc;
  use std::cell::RefCell;
  use std::collections::HashMap;

  #[test]
  fn test_scalar_type() {
    let int_t = GraphQLInt;
    assert_eq!("Int", int_t.name());
    assert_eq!(Some(10), int_t.coerce_literal("10"));
    assert_eq!(None, int_t.coerce_literal("10.1"));
    assert_eq!(None, int_t.coerce_literal("a"));
    assert_eq!(None, int_t.coerce_literal(&i64::max_value().to_string()));
    assert_eq!(None, int_t.coerce_literal(&i64::min_value().to_string()));

    let float_t = GraphQLFloat;
    assert_eq!("Float", float_t.name());
    assert_eq!(Some(2.0), float_t.coerce_literal("2.0"));
    assert_eq!(Some(2.0), float_t.coerce_literal("2"));
    assert_eq!(None, float_t.coerce_literal("2.0a"));

    let string_t = GraphQLString;
    assert_eq!("String", string_t.name());
    assert_eq!(Some(String::from("abc")), string_t.coerce_literal("abc"));
    assert_eq!(Some(String::from("2.0")), string_t.coerce_literal("2.0"));

    let boolean_t = GraphQLBoolean;
    assert_eq!("Boolean", boolean_t.name());
    assert_eq!(Some(true), boolean_t.coerce_literal("true"));
    assert_eq!(Some(false), boolean_t.coerce_literal("false"));
    assert_eq!(None, boolean_t.coerce_literal("1"));
    assert_eq!(None, boolean_t.coerce_literal("0"));
    assert_eq!(None, boolean_t.coerce_literal("True"));
    assert_eq!(None, boolean_t.coerce_literal("False"));
    assert_eq!(None, boolean_t.coerce_literal("TRUE"));
    assert_eq!(None, boolean_t.coerce_literal("FALSE"));
  }

  #[test]
  fn test_enum_type() {
    let rgb = GraphQLEnumBuilder::new("RGB")
                .value("RED", |v| v)
                .value("GREEN", |v| v)
                .value("BLUE", |v| v)
                .build();
    assert_eq!(3, rgb.values.len());

    let days = GraphQLEnumBuilder::new("DAYS")
                 .with_description("Days of the week")
                 .value("SAT", |v| v.with_description("Satarday"))
                 .value("SUN", |v| v.with_description("Sunday"))
                 .value("MON", |v| v.with_description("Monday"))
                 .value("TUE", |v| v.with_description("Tuesday"))
                 .value("WED", |v| v.with_description("Wedsday"))
                 .value("THU", |v| v.with_description("Thusday"))
                 .value("FRI", |v| v.with_description("Friday"))
                 .build();
    assert_eq!(Some("Days of the week"),
               days.description.as_ref().map(|s| s.as_ref()));
    assert_eq!(Some("Monday"),
               days.values["MON"].description.as_ref().map(|s| s.as_ref()));
  }

  #[test]
  fn test_object_type() {
    let INT = &Rc::new(GraphQLInt);
    let FLOAT = &Rc::new(GraphQLFloat);
    let STRING = &Rc::new(GraphQLString);
    let BOOLEAN = &Rc::new(GraphQLBoolean);

    let IMAGE = &GraphQLObjectBuilder::new("Image")
                   .with_description("Image Type")
                   .field("url", |f| f.type_of(STRING))
                   .field("width", |f| f.type_of(INT))
                   .field("height", |f| f.type_of(INT))
                   .build();

    let AUTHOR = &GraphQLObjectBuilder::new("Author")
                    .with_description("Author Type")
                    .field("id", |f| f.type_of(STRING))
                    .field("name", |f| f.type_of(STRING))
                    .field("pic", |f| {
                      f.type_of(IMAGE)
                       .arg("width", |a| a.type_of(INT))
                       .arg("height", |a| a.type_of(INT))
                    })
                    .field("recentArticle", |f| f.placeholder_type_of("Article"))
                    .build();

    let ARTICLE = &GraphQLObjectBuilder::new("Article")
                     .field("id", |f| f.type_of(STRING))
                     .field("isPublished", |f| f.type_of(BOOLEAN))
                     .field("author", |f| f.type_of(AUTHOR))
                     .field("title", |f| f.type_of(STRING))
                     .field("body", |f| f.type_of(STRING))
                     .build();

    AUTHOR.replace_field_placeholder_type("recentArticle", ARTICLE);
    assert_eq!("Article",
               AUTHOR.fields.borrow()["recentArticle"].typ.name());

    let SEARCH_RESULT = &GraphQLUnionBuilder::new("SearchResult")
                           .with_description("Result will be either Author or Article")
                           .maybe_type_of(AUTHOR)
                           .maybe_type_of(ARTICLE)
                           .build();
  }

  #[test]
  fn test_interface_type() {
    let INT = &Rc::new(GraphQLInt);
    let FLOAT = &Rc::new(GraphQLFloat);
    let STRING = &Rc::new(GraphQLString);
    let BOOLEAN = &Rc::new(GraphQLBoolean);

    let NAMED_ENTITY = &GraphQLInterfaceBuilder::new("NamedEntity")
                          .field("name", |f| f.type_of(STRING))
                          .build();

    let PERSON = &GraphQLObjectBuilder::new("Person")
                    .field("name", |f| f.type_of(STRING))
                    .field("age", |f| f.type_of(INT))
                    .impl_interface(NAMED_ENTITY)
                    .build();

    let BUSINESS = &GraphQLObjectBuilder::new("Person")
                      .field("name", |f| f.type_of(STRING))
                      .field("employeeCount", |f| f.type_of(INT))
                      .impl_interface(NAMED_ENTITY)
                      .build();

    assert!(PERSON.interfaces.as_ref().unwrap().contains_key("NamedEntity"));
    assert!(BUSINESS.interfaces.as_ref().unwrap().contains_key("NamedEntity"));
  }
}
