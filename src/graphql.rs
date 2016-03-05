use std::collections::HashMap;

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

pub trait GraphQLScalar: GraphQLType {}
blanket_impl! { GraphQLScalar for GraphQLInt, GraphQLFloat, GraphQLString, GraphQLBoolean }

pub struct GraphQLInt;
impl_graphql_type_for! {
    GraphQLInt where
    name = "Int",
    description = "The Int scalar type represents a signed 32‐bit numeric non‐fractional values."
}

pub struct GraphQLFloat;
impl_graphql_type_for! {
    GraphQLFloat where
    name = "Float",
    description = "The Float scalar type represents signed double-precision fractional values as specified by IEEE 754."
}

pub struct GraphQLString;
impl_graphql_type_for! {
    GraphQLString where
    name = "String",
    description = "The String scalar type represents textual data, represented as UTF-8 character sequences."
}

pub struct GraphQLBoolean;
impl_graphql_type_for! {
    GraphQLBoolean where
    name = "Boolean",
    description = "The Boolean scalar type represents true or false."
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
    if depreciation_reason.trim().len() == 0 {
      panic!("Deprecation reason for {:?} can't not be empty", self.value);
    }
    self.depreciation_reason = Some(depreciation_reason.to_owned());
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
    assert_eq!("Int", GraphQLInt.name());
    assert_eq!("Float", GraphQLFloat.name());
    assert_eq!("String", GraphQLString.name());
    assert_eq!("Boolean", GraphQLBoolean.name());
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
