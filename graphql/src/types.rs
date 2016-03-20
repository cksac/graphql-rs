use std::collections::HashMap;
use std::str::FromStr;
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
impl_graphql_type_for! { GraphQLObject, GraphQLInterface, GraphQLUnion, GraphQLEnum, GraphQLInputObject, GraphQLList, GraphQLInputList, GraphQLInputNonNull, GraphQLNonNull }

pub trait GraphQLInput: GraphQLType {}
impl<T: GraphQLScalar> GraphQLInput for T {}
blanket_impl! { GraphQLInput for GraphQLEnum, GraphQLInputObject, GraphQLInputList, GraphQLInputNonNull }

pub trait GraphQLOutput: GraphQLType {}
impl<T: GraphQLScalar> GraphQLOutput for T {}
blanket_impl! { GraphQLOutput for GraphQLObject, GraphQLInterface, GraphQLUnion, GraphQLEnum, GraphQLList, GraphQLNonNull }

/// Scalars
pub trait GraphQLScalar: GraphQLType {
  type ValueType;
  fn coerce_literal(&self, value: &str) -> Option<Self::ValueType>;
}

/// Built-in Scalars
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

/// Object
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
    let field = self.fields.borrow_mut().remove(field_name);
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
  deprecation_reason: Option<String>,
  typ: Rc<GraphQLOutput>,
  args: Option<HashMap<String, GraphQLArgument>>,
}

pub struct GraphQLArgument {
  name: String,
  description: Option<String>,
  typ: Rc<GraphQLInput>,
}

/// Interfaces
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
      panic!("Interface type {:} does not have placeholder {:} field.",
             self.name,
             field_name);
    }

    if let Some(mut f) = field {
      let f_type_name = f.typ.name().to_owned();
      if !f_type_name.ends_with("___TypePlaceholder___") {
        panic!("Field {:} in interface type {:} is not a placeholder.",
               field_name,
               self.name);
      }

      let target_type = f_type_name.trim_right_matches("___TypePlaceholder___");
      if target_type != other_type.name() {
        panic!("Placeholder {:} in interface type {:} should replaced by {:} type instead of \
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

/// Union
pub struct GraphQLUnion {
  name: String,
  description: Option<String>,
  types: HashMap<String, Rc<GraphQLObject>>,
}

/// Enum
pub struct GraphQLEnum {
  name: String,
  description: Option<String>,
  values: HashMap<String, GraphQLEnumValue>,
}

pub struct GraphQLEnumValue {
  value: String,
  description: Option<String>,
  deprecation_reason: Option<String>,
}

/// Input Object
pub struct GraphQLInputObject {
  name: String,
  description: Option<String>,
  fields: RefCell<HashMap<String, GraphQLInputField>>,
}

pub struct GraphQLInputField {
  name: String,
  description: Option<String>,
  typ: Rc<GraphQLInput>,
}

/// List
pub struct GraphQLInputList {
  name: String,
  description: Option<String>,
  of_typ: Rc<GraphQLInput>,
}

pub struct GraphQLList {
  name: String,
  description: Option<String>,
  of_typ: Rc<GraphQLOutput>,
}

/// Non-Null
pub struct GraphQLInputNonNull {
  name: String,
  description: Option<String>,
  of_typ: Rc<GraphQLInput>,
}

pub struct GraphQLNonNull {
  name: String,
  description: Option<String>,
  of_typ: Rc<GraphQLOutput>,
}

// /////////////////////////////////////////////////////////////////////////////
// Type Builders
// /////////////////////////////////////////////////////////////////////////////

// Internal type placeholder for forward reference to other graphql type.
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

impl GraphQLOutput for Placeholder {}

/// Scalar type builder
pub struct GraphQLScalarType;

impl GraphQLScalarType {
  pub fn int() -> Rc<GraphQLInt> {
    Rc::new(GraphQLInt)
  }

  pub fn float() -> Rc<GraphQLFloat> {
    Rc::new(GraphQLFloat)
  }

  pub fn string() -> Rc<GraphQLString> {
    Rc::new(GraphQLString)
  }

  pub fn boolean() -> Rc<GraphQLBoolean> {
    Rc::new(GraphQLBoolean)
  }

  pub fn custom<T, F>(f: F) -> Rc<T>
    where T: GraphQLScalar,
          F: Fn() -> T
  {
    Rc::new(f())
  }
}

/// Object type builder
pub struct GraphQLObjectType {
  name: String,
  description: Option<String>,
  fields: HashMap<String, GraphQLField>,
  interfaces: Option<HashMap<String, Rc<GraphQLInterface>>>,
}

impl GraphQLObjectType {
  pub fn new(name: &str) -> GraphQLObjectType {
    GraphQLObjectType {
      name: name.to_owned(),
      description: None,
      fields: HashMap::new(),
      interfaces: None,
    }
  }

  pub fn description(mut self, description: &str) -> GraphQLObjectType {
    self.description = Some(description.to_owned());
    self
  }

  pub fn field<F>(mut self, name: &str, f: F) -> GraphQLObjectType
    where F: Fn(GraphQLFieldBuilder) -> GraphQLFieldBuilder
  {
    let field = f(GraphQLFieldBuilder::new(name)).build();
    self.fields.insert(name.to_owned(), field);
    self
  }

  pub fn impl_interface(mut self, interface: &Rc<GraphQLInterface>) -> GraphQLObjectType {
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

/// interface type builder
pub struct GraphQLInterfaceType {
  name: String,
  description: Option<String>,
  fields: HashMap<String, GraphQLField>,
}

impl GraphQLInterfaceType {
  pub fn new(name: &str) -> GraphQLInterfaceType {
    GraphQLInterfaceType {
      name: name.to_owned(),
      description: None,
      fields: HashMap::new(),
    }
  }

  pub fn description(mut self, description: &str) -> GraphQLInterfaceType {
    self.description = Some(description.to_owned());
    self
  }

  pub fn field<F>(mut self, name: &str, f: F) -> GraphQLInterfaceType
    where F: Fn(GraphQLFieldBuilder) -> GraphQLFieldBuilder
  {
    let field = f(GraphQLFieldBuilder::new(name)).build();
    self.fields.insert(name.to_owned(), field);
    self
  }

  pub fn build(self) -> Rc<GraphQLInterface> {
    if self.fields.len() == 0 {
      panic!("Interface type {:} must contains at least one field",
             self.name);
    }

    Rc::new(GraphQLInterface {
      name: self.name,
      description: self.description,
      fields: RefCell::new(self.fields),
    })
  }
}

pub struct GraphQLFieldBuilder {
  name: String,
  description: Option<String>,
  deprecation_reason: Option<String>,
  typ: Option<Rc<GraphQLOutput>>,
  args: Option<HashMap<String, GraphQLArgument>>,
}

impl GraphQLFieldBuilder {
  fn new(name: &str) -> GraphQLFieldBuilder {
    GraphQLFieldBuilder {
      name: name.to_owned(),
      description: None,
      deprecation_reason: None,
      typ: None,
      args: None,
    }
  }

  pub fn description(mut self, description: &str) -> GraphQLFieldBuilder {
    self.description = Some(description.to_owned());
    self
  }

  pub fn mark_deprecated(mut self, reason: &str) -> GraphQLFieldBuilder {
    self.deprecation_reason = Some(reason.to_owned());
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

  fn build(self) -> GraphQLField {
    if self.typ.is_none() {
      panic!("Field {:} missing type defination", self.name);
    }

    GraphQLField {
      name: self.name,
      description: self.description,
      deprecation_reason: self.deprecation_reason,
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
  fn new(name: &str) -> GraphQLArgumentBuilder {
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

  fn build(self) -> GraphQLArgument {
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

/// Union type builder
pub struct GraphQLUnionType {
  name: String,
  description: Option<String>,
  types: HashMap<String, Rc<GraphQLObject>>,
}

impl GraphQLUnionType {
  pub fn new(name: &str) -> GraphQLUnionType {
    GraphQLUnionType {
      name: name.to_owned(),
      description: None,
      types: HashMap::new(),
    }
  }

  pub fn description(mut self, description: &str) -> GraphQLUnionType {
    self.description = Some(description.to_owned());
    self
  }

  pub fn maybe_type_of(mut self, typ: &Rc<GraphQLObject>) -> GraphQLUnionType {
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

/// Enum type builder
pub struct GraphQLEnumType {
  name: String,
  description: Option<String>,
  values: HashMap<String, GraphQLEnumValue>,
}

impl GraphQLEnumType {
  pub fn new(name: &str) -> GraphQLEnumType {
    GraphQLEnumType {
      name: name.to_owned(),
      description: None,
      values: HashMap::new(),
    }
  }

  pub fn description(mut self, description: &str) -> GraphQLEnumType {
    self.description = Some(description.to_owned());
    self
  }

  pub fn value<F>(mut self, name: &str, f: F) -> GraphQLEnumType
    where F: Fn(GraphQLEnumValueBuilder) -> GraphQLEnumValueBuilder
  {
    let v = f(GraphQLEnumValueBuilder::new(name)).build();
    self.values.insert(name.to_owned(), v);
    self
  }

  pub fn build(self) -> Rc<GraphQLEnum> {
    if self.values.len() == 0 {
      panic!("Enum {:} must has at least one value defined.", self.name);
    }

    Rc::new(GraphQLEnum {
      name: self.name,
      description: self.description,
      values: self.values,
    })
  }
}

pub struct GraphQLEnumValueBuilder {
  value: String,
  description: Option<String>,
  deprecation_reason: Option<String>,
}

impl GraphQLEnumValueBuilder {
  fn new(value: &str) -> GraphQLEnumValueBuilder {
    GraphQLEnumValueBuilder {
      value: value.to_owned(),
      description: None,
      deprecation_reason: None,
    }
  }

  pub fn description(mut self, description: &str) -> GraphQLEnumValueBuilder {
    self.description = Some(description.to_owned());
    self
  }

  pub fn mark_deprecated(mut self, deprecation_reason: &str) -> GraphQLEnumValueBuilder {
    let reason = deprecation_reason.trim().to_owned();
    if reason.len() == 0 {
      panic!("Deprecation reason for enum value {:} cannot be empty",
             self.value);
    }
    self.deprecation_reason = Some(reason);
    self
  }

  fn build(self) -> GraphQLEnumValue {
    GraphQLEnumValue {
      value: self.value,
      description: self.description,
      deprecation_reason: self.deprecation_reason,
    }
  }
}

/// Input object type builder
pub struct GraphQLInputObjectType {
  name: String,
  description: Option<String>,
  fields: HashMap<String, GraphQLInputField>,
}

impl GraphQLInputObjectType {
  pub fn new(name: &str) -> GraphQLInputObjectType {
    GraphQLInputObjectType {
      name: name.to_owned(),
      description: None,
      fields: HashMap::new(),
    }
  }

  pub fn description(mut self, description: &str) -> GraphQLInputObjectType {
    self.description = Some(description.to_owned());
    self
  }

  pub fn field<F>(mut self, name: &str, f: F) -> GraphQLInputObjectType
    where F: Fn(GraphQLInputFieldBuilder) -> GraphQLInputFieldBuilder
  {
    let field = f(GraphQLInputFieldBuilder::new(name)).build();
    self.fields.insert(name.to_owned(), field);
    self
  }

  pub fn build(self) -> Rc<GraphQLInputObject> {
    if self.fields.len() == 0 {
      panic!("Input object type {:} must contains at least one field",
             self.name);
    }

    Rc::new(GraphQLInputObject {
      name: self.name,
      description: self.description,
      fields: RefCell::new(self.fields),
    })
  }
}

pub struct GraphQLInputFieldBuilder {
  name: String,
  description: Option<String>,
  typ: Option<Rc<GraphQLInput>>,
}

impl GraphQLInputFieldBuilder {
  fn new(name: &str) -> GraphQLInputFieldBuilder {
    GraphQLInputFieldBuilder {
      name: name.to_owned(),
      description: None,
      typ: None,
    }
  }

  pub fn type_of<T: GraphQLInput + 'static>(mut self, typ: &Rc<T>) -> GraphQLInputFieldBuilder {
    self.typ = Some(typ.clone());
    self
  }

  fn build(self) -> GraphQLInputField {
    if self.typ.is_none() {
      panic!("Input object field {:} missing type defination", self.name);
    }

    GraphQLInputField {
      name: self.name,
      description: self.description,
      typ: self.typ.unwrap(),
    }
  }
}

/// List type builder
pub struct GraphQLListType;
impl GraphQLListType {
  pub fn input<T: GraphQLInput + 'static>(of_type: &Rc<T>) -> Rc<GraphQLInputList> {
    Rc::new(GraphQLInputList {
      name: of_type.name().to_owned(),
      description: Some(format!("List of {}", of_type.name())),
      of_typ: of_type.clone(),
    })
  }

  pub fn output<T: GraphQLOutput + 'static>(of_type: &Rc<T>) -> Rc<GraphQLList> {
    Rc::new(GraphQLList {
      name: of_type.name().to_owned(),
      description: Some(format!("List of {}", of_type.name())),
      of_typ: of_type.clone(),
    })
  }
}

/// Non-null type builder
pub struct GraphQLNonNullType;
impl GraphQLNonNullType {
  pub fn input<T: GraphQLInput + 'static>(of_type: &Rc<T>) -> Rc<GraphQLInputNonNull> {
    Rc::new(GraphQLInputNonNull {
      name: of_type.name().to_owned(),
      description: Some(format!("Non-null {}", of_type.name())),
      of_typ: of_type.clone(),
    })
  }

  pub fn output<T: GraphQLOutput + 'static>(of_type: &Rc<T>) -> Rc<GraphQLNonNull> {
    Rc::new(GraphQLNonNull {
      name: of_type.name().to_owned(),
      description: Some(format!("Non-null {}", of_type.name())),
      of_typ: of_type.clone(),
    })
  }
}
