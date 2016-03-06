use std::collections::HashMap;
use std::str::FromStr;
use std::any::Any;

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

pub trait GraphQLType {
  fn name(&self) -> &str;
  fn description(&self) -> Option<&str>;
}
impl_graphql_type_for! { GraphQLEnum }

pub trait GraphQLScalar: GraphQLType {
  type ValueType: Any;
  fn coerce_literal(&self, value: &str) -> Option<Self::ValueType>;
}

pub struct GraphQLInt;
impl_graphql_type_for! {
    GraphQLInt where
    name = "Int",
    description = "The Int scalar type represents a signed 32‐bit numeric non‐fractional values."
}

impl GraphQLScalar for GraphQLInt {
  type ValueType = i32;
  fn coerce_literal(&self, value: &str) -> Option<Self::ValueType> {
    i32::from_str(value).ok()
  }
}

pub struct GraphQLFloat;
impl_graphql_type_for! {
    GraphQLFloat where
    name = "Float",
    description = "The Float scalar type represents signed double-precision fractional values as specified by IEEE 754."
}

impl GraphQLScalar for GraphQLFloat {
  type ValueType = f64;
  fn coerce_literal(&self, value: &str) -> Option<Self::ValueType> {
    f64::from_str(value).ok()
  }
}

pub struct GraphQLString;
impl_graphql_type_for! {
    GraphQLString where
    name = "String",
    description = "The String scalar type represents textual data, represented as UTF-8 character sequences."
}

impl GraphQLScalar for GraphQLString {
  type ValueType = String;
  fn coerce_literal(&self, value: &str) -> Option<Self::ValueType> {
    String::from_str(value).ok()
  }
}

pub struct GraphQLBoolean;
impl_graphql_type_for! {
    GraphQLBoolean where
    name = "Boolean",
    description = "The Boolean scalar type represents true or false."
}

impl GraphQLScalar for GraphQLBoolean {
  type ValueType = bool;
  fn coerce_literal(&self, value: &str) -> Option<Self::ValueType> {
    bool::from_str(value).ok()
  }
}

pub struct GraphQLEnum {
  name: String,
  description: Option<String>,
  values: HashMap<String, GraphQLEnumValue>,
}

impl GraphQLEnum {
  fn values(&self) -> &HashMap<String, GraphQLEnumValue> {
    &self.values
  }
}

pub struct GraphQLEnumValue {
  value: String,
  description: Option<String>,
  depreciation_reason: Option<String>,
}

impl GraphQLEnumValue {
  pub fn new(value: &str) -> GraphQLEnumValue {
    GraphQLEnumValue {
      value: value.to_owned(),
      description: None,
      depreciation_reason: None,
    }
  }

  pub fn value(&self) -> &str {
    self.value.as_ref()
  }

  pub fn description(&self) -> Option<&str> {
    self.description.as_ref().map(|s| s.as_ref())
  }

  pub fn with_description(mut self, description: &str) -> GraphQLEnumValue {
    self.description = Some(description.to_owned());
    self
  }

  pub fn mark_depreciated(mut self, depreciation_reason: &str) -> GraphQLEnumValue {
    let reason = depreciation_reason.trim().to_owned();
    if reason.len() == 0 {
      panic!("Deprecation reason for enum value {:?} cannot be empty",
             self.value);
    }
    self.depreciation_reason = Some(reason);
    self
  }
}

pub struct GraphQLEnumBuilder {
  name: String,
  description: Option<String>,
  values: HashMap<String, GraphQLEnumValue>,
}

impl GraphQLEnumBuilder {
  fn new(name: &str) -> GraphQLEnumBuilder {
    GraphQLEnumBuilder {
      name: name.to_owned(),
      description: None,
      values: HashMap::new(),
    }
  }

  fn with_description(mut self, description: &str) -> GraphQLEnumBuilder {
    self.description = Some(description.to_owned());
    self
  }

  fn value_fn<F>(mut self, name: &str, f: F) -> GraphQLEnumBuilder
    where F: FnOnce(GraphQLEnumValue) -> GraphQLEnumValue
  {
    let v = f(GraphQLEnumValue::new(name));
    self.values.insert(v.value.clone(), v);
    self
  }

  fn value(mut self, name: &str) -> GraphQLEnumBuilder {
    self.values.insert(name.to_owned(), GraphQLEnumValue::new(name));
    self
  }

  fn build(self) -> GraphQLEnum {
    if self.values.len() == 0 {
      panic!("Enum {:?} must has at least one value defined.", self.name);
    }

    GraphQLEnum {
      name: self.name,
      description: self.description,
      values: self.values,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

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
                .value("RED")
                .value("GREEN")
                .value("BLUE")
                .build();
    assert_eq!(3, rgb.values().len());

    let days = GraphQLEnumBuilder::new("DAYS")
                 .with_description("Days of the week")
                 .value_fn("SAT", |v| v.with_description("Satarday"))
                 .value_fn("SUN", |v| v.with_description("Sunday"))
                 .value_fn("MON", |v| v.with_description("Monday"))
                 .value_fn("TUE", |v| v.with_description("Tuesday"))
                 .value_fn("WED", |v| v.with_description("Wedsday"))
                 .value_fn("THU", |v| v.with_description("Thusday"))
                 .value_fn("FRI", |v| v.with_description("Friday"))
                 .build();
    assert_eq!("Days of the week", days.description().unwrap_or(""));
    assert_eq!("Monday", days.values()["MON"].description().unwrap_or(""));;
  }
}
