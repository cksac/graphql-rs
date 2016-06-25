use super::*;

/// Value[Const] :
///   - [~Const] Variable
///   - IntValue
///   - FloatValue
///   - StringValue
///   - BooleanValue
///   - EnumValue
///   - ListValue[?Const]
///   - ObjectValue[?Const]
pub enum Value {
  Variable(Variable),
  Int(IntValue),
  Float(FloatValue),
  String(StringValue),
  Boolean(BooleanValue),
  Enum(EnumValue),
  List(ListValue),
  Object(ObjectValue),
}

/// DefaultValue : = Value[Const]
pub type DefaultValue = Value;

/// Variable : $ Name
pub type Variable = Name;

/// Name :: /[_A-Za-z][_0-9A-Za-z]*/
pub struct Name {
  pub loc: Option<Location>,
  pub value: String,
}

impl_node_for! { Name }

/// Alias : Name :
pub type Alias = Name;

/// Arguments : ( Argument+ )
pub type Arguments = Vec<Argument>;

/// Argument : Name : Value
pub struct Argument {
  pub loc: Option<Location>,
  pub name: Name,
  pub value: Value,
}

impl_node_for! { Argument }

/// IntValue :: IntegerPart
pub struct IntValue {
  pub loc: Option<Location>,
  pub value: usize,
}

impl_node_for! { IntValue }

/// FloatValue ::
///   - IntegerPart FractionalPart
///   - IntegerPart ExponentPart
///   - IntegerPart FractionalPart ExponentPart
pub struct FloatValue {
  pub loc: Option<Location>,
  pub value: f64,
}

impl_node_for! { FloatValue }

/// StringValue ::
///   - `""`
///   - `"` StringCharacter+ `"`
pub struct StringValue {
  pub loc: Option<Location>,
  pub value: String,
}

impl_node_for! { StringValue }

/// BooleanValue : one of `true` `false`
pub struct BooleanValue {
  pub loc: Option<Location>,
  pub value: bool,
}

impl_node_for! { BooleanValue }

/// EnumValue : Name but not `true`, `false` or `null`
pub type EnumValue = Name;

/// ListValue[Const] :
///   - [ ]
///   - [ Value[?Const]+ ]
pub struct ListValue {
  pub loc: Option<Location>,
  pub values: Vec<Value>,
}

impl_node_for! { ListValue }

/// ObjectValue[Const] :
///   - { }
///   - { ObjectField[?Const]+ }
pub struct ObjectValue {
  pub loc: Option<Location>,
  pub fields: Vec<ObjectField>,
}

impl_node_for! { ObjectValue }

/// ObjectField[Const] : Name : Value[?Const]
pub struct ObjectField {
  pub loc: Option<Location>,
  pub name: Name,
  pub value: Value,
}

impl_node_for! { ObjectField }
