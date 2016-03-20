use lexer::Token::{Name, Int as IntValue, Float as FloatValue, String as StringValue};

/// Document : Definition+
pub struct Document {
  pub definitions: Vec<Definition>
}

/// Definition :
///   - OperationDefinition
///   - FragmentDefinition
pub enum Definition {
  Operation(OperationDefinition),
  Fragment(FragmentDefinition)
}

/// OperationDefinition :
///   - SelectionSet
///   - OperationType Name? VariableDefinitions? Directives? SelectionSet
pub struct OperationDefinition {
  pub operation_type: OperationType,
  pub name: Option<Name>,
  pub variable_definitions: Option<VariableDefinitions>,
  pub directives: Option<Directives>,
  pub selection_set: SelectionSet
}

/// OperationType : one of query mutation
pub enum OperationType {
  Query,
  Mutation
}

/// SelectionSet : { Selection+ }
pub type SelectionSet = Vec<Selection>;

/// Selection :
///   - Field
///   - FragmentSpread
///   - InlineFragment
pub enum Selection {
  Field(Field),
  FragmentSpread(FragmentSpread),
  InlineFragment(InlineFragment)
}

/// Field : Alias? Name Arguments? Directives? SelectionSet?
pub struct Field {
  pub alias: Option<Alias>,
  pub name: Name,
  pub arguments: Option<Arguments>,
  pub directives: Option<Directives>,
  pub selection_set: Option<SelectionSet>
}

/// Alias : Name :
pub type Alias = Name;

/// Arguments : ( Argument+ )
pub type Arguments = Vec<Argument>;

/// Argument : Name : Value
pub struct Argument {
  pub name: Name,
  pub value: Value
}

/// FragmentSpread : ... FragmentName Directives?
pub struct FragmentSpread {
  pub fragment_name: Name,
  pub directives: Option<Directives>
}

/// InlineFragment : ... TypeCondition? Directives? SelectionSet
pub struct InlineFragment {
  pub type_condition: Option<TypeCondition>,
  pub directives: Option<Directives>,
  pub selection_set: SelectionSet
}

/// FragmentDefinition : fragment FragmentName TypeCondition Directives? SelectionSet
pub struct FragmentDefinition {
  pub fragment_name: Name,
  pub type_condition: TypeCondition,
  pub directives: Option<Directives>,
  pub selection_set: SelectionSet
}

/// FragmentName : Name but not `on`
pub type FragmentName = Name;

/// TypeCondition : on NamedType
pub type TypeCondition = NamedType;

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
  Object(ObjectValue)
}

/// BooleanValue : one of `true` `false`
pub enum BooleanValue {
  True,
  False
}

/// EnumValue : Name but not `true`, `false` or `null`
pub struct EnumValue {
  pub name: Name
}

/// ListValue[Const] :
///   - [ ]
///   - [ Value[?Const]+ ]
pub type ListValue = Vec<Value>;

/// ObjectValue[Const] :
///   - { }
///   - { ObjectField[?Const]+ }
pub type ObjectValue = Vec<ObjectField>;

/// ObjectField[Const] : Name : Value[?Const]
pub struct ObjectField {
  pub name: Name,
  pub value: Value
}

/// VariableDefinitions : ( VariableDefinition+ )
pub type VariableDefinitions = Vec<VariableDefinition>;

/// VariableDefinition : Variable : Type DefaultValue?
pub struct VariableDefinition {
  pub variable: Variable,
  pub type_: Type,
  pub default_value: Option<DefaultValue>
}

/// Variable : $ Name
pub type Variable = Name;

/// DefaultValue : = Value[Const]
pub type DefaultValue = Value;

/// Type :
///   - NamedType
///   - ListType
///   - NonNullType
pub enum Type {
  Named(NamedType),
  List(ListType),
  NonNull(NonNullType)
}

/// NamedType : Name
pub type NamedType = Name;

/// ListType : [ Type ]
pub type ListType = Name;

/// NonNullType :
///   - NamedType !
///   - ListType !
pub enum NonNullType {
  Named(NamedType),
  List(ListType)
}

/// Directives : Directive+
pub type Directives = Vec<Directive>;

/// Directive : @ Name Arguments?
pub struct Directive {
  pub name: Name,
  pub arguments: Option<Arguments>
}
