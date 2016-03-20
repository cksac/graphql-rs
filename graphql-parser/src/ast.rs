//! AST node definitions which will be parsed into. Based off of the
//! `graphql-js` [`ast.js`][1] definitions.
//!
//! [1]: https://github.com/graphql/graphql-js/blob/dfe676c3011efe9560b9fa0fcbd2b7bd87476d02/src/language/ast.js

/// All AST node types implement this trait.
trait Node {}

macro_rules! impl_node_for {
  ($struct:ident) => {
    impl Node for $struct {}
  }
}

/// Name :: /[_A-Za-z][_0-9A-Za-z]*/
pub struct Name {
  pub value: String
}

impl_node_for! { Name }

/// IntValue :: IntegerPart
pub struct IntValue {
  pub value: String
}

impl_node_for! { IntValue }

/// FloatValue ::
///   - IntegerPart FractionalPart
///   - IntegerPart ExponentPart
///   - IntegerPart FractionalPart ExponentPart
pub struct FloatValue {
  pub value: String
}

impl_node_for! { FloatValue }

/// StringValue ::
///   - `""`
///   - `"` StringCharacter+ `"`
pub struct StringValue {
  pub value: String
}

impl_node_for! { StringValue }

/// Document : Definition+
pub struct Document {
  pub definitions: Vec<Definition>
}

impl_node_for! { Document }

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
  pub operation: OperationType,
  pub name: Option<Name>,
  pub variable_definitions: Option<VariableDefinitions>,
  pub directives: Option<Directives>,
  pub selection_set: SelectionSet
}

impl_node_for! { OperationDefinition }

/// OperationType : one of query mutation
pub enum OperationType {
  Query,
  Mutation
}

/// SelectionSet : { Selection+ }
pub struct SelectionSet {
  pub selections: Vec<Selection>
}

impl_node_for! { SelectionSet }

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

impl_node_for! { Field }

/// Alias : Name :
pub type Alias = Name;

/// Arguments : ( Argument+ )
pub type Arguments = Vec<Argument>;

/// Argument : Name : Value
pub struct Argument {
  pub name: Name,
  pub value: Value
}

impl_node_for! { Argument }

/// FragmentSpread : ... FragmentName Directives?
pub struct FragmentSpread {
  pub name: Name,
  pub directives: Option<Directives>
}

impl_node_for! { FragmentSpread }

/// InlineFragment : ... TypeCondition? Directives? SelectionSet
pub struct InlineFragment {
  pub type_condition: Option<TypeCondition>,
  pub directives: Option<Directives>,
  pub selection_set: SelectionSet
}

impl_node_for! { InlineFragment }

/// FragmentDefinition : fragment FragmentName TypeCondition Directives? SelectionSet
pub struct FragmentDefinition {
  pub name: Name,
  pub type_condition: TypeCondition,
  pub directives: Option<Directives>,
  pub selection_set: SelectionSet
}

impl_node_for! { FragmentDefinition }

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

impl_node_for! { BooleanValue }

/// EnumValue : Name but not `true`, `false` or `null`
pub struct EnumValue {
  pub name: Name
}

impl_node_for! { EnumValue }

/// ListValue[Const] :
///   - [ ]
///   - [ Value[?Const]+ ]
pub struct ListValue {
  pub values: Vec<Value>
}

impl_node_for! { ListValue }

/// ObjectValue[Const] :
///   - { }
///   - { ObjectField[?Const]+ }
pub struct ObjectValue {
  pub fields: Vec<ObjectField>
}

impl_node_for! { ObjectValue }

/// ObjectField[Const] : Name : Value[?Const]
pub struct ObjectField {
  pub name: Name,
  pub value: Value
}

impl_node_for! { ObjectField }

/// VariableDefinitions : ( VariableDefinition+ )
pub type VariableDefinitions = Vec<VariableDefinition>;

/// VariableDefinition : Variable : Type DefaultValue?
pub struct VariableDefinition {
  pub variable: Variable,
  pub type_: Type,
  pub default_value: Option<DefaultValue>
}

impl_node_for! { VariableDefinition }

/// Variable : $ Name
pub struct Variable {
  pub name: Name
}

impl_node_for! { Variable }

/// DefaultValue : = Value[Const]
pub type DefaultValue = Value;

/// Type :
///   - NamedType
///   - ListType
///   - NonNullType
pub enum Type {
  Named(NamedType),
  List(ListType),
  NonNullNamed(NonNullNamedType),
  NonNullList(NonNullListType)
}

/// NamedType : Name
pub struct NamedType {
  pub name: Name
}

impl_node_for! { NamedType }

/// ListType : [ Type ]
pub struct ListType {
  pub type_: Type
}

impl_node_for! { ListType }

/// NonNullType :
///   - NamedType !
///   - ListType !
///
/// Are implementation deviates from the spec here. This is because
/// `NonNullType` is expected to be a [node][1], but it is also expected to be
/// a [union][2]. The best way to express this in Rust is with two types.
///
/// [1]: https://github.com/graphql/graphql-js/blob/dfe676c3011efe9560b9fa0fcbd2b7bd87476d02/src/language/ast.js#L49
/// [2]: https://github.com/graphql/graphql-js/blob/dfe676c3011efe9560b9fa0fcbd2b7bd87476d02/src/language/ast.js#L254
pub struct NonNullNamedType {
  pub type_: NamedType
}

impl_node_for! { NonNullNamedType }

/// See documentation for the `NonNullNamedType` struct.
pub struct NonNullListType {
  pub type_: ListType
}

impl_node_for! { NonNullListType }

/// Directives : Directive+
pub type Directives = Vec<Directive>;

/// Directive : @ Name Arguments?
pub struct Directive {
  pub name: Name,
  pub arguments: Option<Arguments>
}

impl_node_for! { Directive }
