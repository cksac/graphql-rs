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
pub enum Value<'a> {
  Variable(Variable<'a>),
  Int(IntValue),
  Float(FloatValue),
  String(StringValue),
  Boolean(BooleanValue),
  Enum(EnumValue<'a>),
  List(ListValue<'a>),
  Object(ObjectValue<'a>),
}

/// DefaultValue : = Value[Const]
pub type DefaultValue<'a> = Value<'a>;

/// Variable : $ Name
pub type Variable<'a> = Name<'a>;

/// Name :: /[_A-Za-z][_0-9A-Za-z]*/
pub struct Name<'a> {
  pub loc: Option<Location>,
  pub value: &'a str,
}

impl_life_node_for! { Name }

/// Alias : Name :
pub type Alias<'a> = Name<'a>;

/// Arguments : ( Argument+ )
pub type Arguments<'a> = Vec<Argument<'a>>;

/// Argument : Name : Value
pub struct Argument<'a> {
  pub loc: Option<Location>,
  pub name: Name<'a>,
  pub value: Value<'a>,
}

impl_life_node_for! { Argument }

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
pub type EnumValue<'a> = Name<'a>;

/// ListValue[Const] :
///   - [ ]
///   - [ Value[?Const]+ ]
pub struct ListValue<'a> {
  pub loc: Option<Location>,
  pub values: Vec<Value<'a>>,
}

impl_life_node_for! { ListValue }

/// ObjectValue[Const] :
///   - { }
///   - { ObjectField[?Const]+ }
pub struct ObjectValue<'a> {
  pub loc: Option<Location>,
  pub fields: Vec<ObjectField<'a>>,
}

impl_life_node_for! { ObjectValue }

/// ObjectField[Const] : Name : Value[?Const]
pub struct ObjectField<'a> {
  pub loc: Option<Location>,
  pub name: Name<'a>,
  pub value: Value<'a>,
}

impl_life_node_for! { ObjectField }
