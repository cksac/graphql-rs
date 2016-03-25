//! AST node definitions which will be parsed into. Based off of the
//! `graphql-js` [`ast.js`][1] definitions.
//!
//! [1]: https://github.com/graphql/graphql-js/blob/dfe676c3011efe9560b9fa0fcbd2b7bd87476d02/src/language/ast.js

use source::Source;

/// Contains some character offsets that identify where the source of the AST
/// is from.
pub struct Location<'a> {
  pub start: usize,
  pub end: usize,
  pub source: Option<&'a Source<'a>>
}

/// All AST node types implement this trait.
pub trait Node<'a> {
  fn location(&self) -> Option<&Location<'a>>;
}

macro_rules! impl_node_for {
  ($data:ident) => {
    impl<'a> Node<'a> for $data<'a> {
      fn location(&self) -> Option<&Location<'a>> {
        self.loc.as_ref()
      }
    }
  }
}

/// Name :: /[_A-Za-z][_0-9A-Za-z]*/
pub struct Name<'a> {
  pub loc: Option<Location<'a>>,
  pub value: &'a str
}

impl_node_for! { Name }

/// IntValue :: IntegerPart
pub struct IntValue<'a> {
  pub loc: Option<Location<'a>>,
  pub value: &'a str
}

impl_node_for! { IntValue }

/// FloatValue ::
///   - IntegerPart FractionalPart
///   - IntegerPart ExponentPart
///   - IntegerPart FractionalPart ExponentPart
pub struct FloatValue<'a> {
  pub loc: Option<Location<'a>>,
  pub value: &'a str
}

impl_node_for! { FloatValue }

/// StringValue ::
///   - `""`
///   - `"` StringCharacter+ `"`
pub struct StringValue<'a> {
  pub loc: Option<Location<'a>>,
  pub value: String
}

impl_node_for! { StringValue }

/// Document : Definition+
pub struct Document<'a> {
  pub loc: Option<Location<'a>>,
  pub definitions: Vec<Definition<'a>>
}

impl_node_for! { Document }

/// Definition :
///   - OperationDefinition
///   - FragmentDefinition
pub enum Definition<'a> {
  Operation(OperationDefinition<'a>),
  Fragment(FragmentDefinition<'a>)
}

/// OperationDefinition :
///   - SelectionSet
///   - OperationType Name? VariableDefinitions? Directives? SelectionSet
pub struct OperationDefinition<'a> {
  pub loc: Option<Location<'a>>,
  pub operation: OperationType,
  pub name: Option<Name<'a>>,
  pub variable_definitions: Option<VariableDefinitions<'a>>,
  pub directives: Option<Directives<'a>>,
  pub selection_set: SelectionSet<'a>
}

impl_node_for! { OperationDefinition }

/// OperationType : one of query mutation
pub enum OperationType {
  Query,
  Mutation
}

/// SelectionSet : { Selection+ }
pub struct SelectionSet<'a> {
  pub loc: Option<Location<'a>>,
  pub selections: Vec<Selection<'a>>
}

impl_node_for! { SelectionSet }

/// Selection :
///   - Field
///   - FragmentSpread
///   - InlineFragment
pub enum Selection<'a> {
  Field(Field<'a>),
  FragmentSpread(FragmentSpread<'a>),
  InlineFragment(InlineFragment<'a>)
}

/// Field : Alias? Name Arguments? Directives? SelectionSet?
pub struct Field<'a> {
  pub loc: Option<Location<'a>>,
  pub alias: Option<Alias<'a>>,
  pub name: Name<'a>,
  pub arguments: Option<Arguments<'a>>,
  pub directives: Option<Directives<'a>>,
  pub selection_set: Option<SelectionSet<'a>>
}

impl_node_for! { Field }

/// Alias : Name :
pub type Alias<'a> = Name<'a>;

/// Arguments : ( Argument+ )
pub type Arguments<'a> = Vec<Argument<'a>>;

/// Argument : Name : Value
pub struct Argument<'a> {
  pub loc: Option<Location<'a>>,
  pub name: Name<'a>,
  pub value: Value<'a>
}

impl_node_for! { Argument }

/// FragmentSpread : ... FragmentName Directives?
pub struct FragmentSpread<'a> {
  pub loc: Option<Location<'a>>,
  pub name: Name<'a>,
  pub directives: Option<Directives<'a>>
}

impl_node_for! { FragmentSpread }

/// InlineFragment : ... TypeCondition? Directives? SelectionSet
pub struct InlineFragment<'a> {
  pub loc: Option<Location<'a>>,
  pub type_condition: Option<TypeCondition<'a>>,
  pub directives: Option<Directives<'a>>,
  pub selection_set: SelectionSet<'a>
}

impl_node_for! { InlineFragment }

/// FragmentDefinition : fragment FragmentName TypeCondition Directives? SelectionSet
pub struct FragmentDefinition<'a> {
  pub loc: Option<Location<'a>>,
  pub name: Name<'a>,
  pub type_condition: TypeCondition<'a>,
  pub directives: Option<Directives<'a>>,
  pub selection_set: SelectionSet<'a>
}

impl_node_for! { FragmentDefinition }

/// FragmentName : Name but not `on`
pub type FragmentName<'a> = Name<'a>;

/// TypeCondition : on NamedType
pub type TypeCondition<'a> = NamedType<'a>;

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
  Int(IntValue<'a>),
  Float(FloatValue<'a>),
  String(StringValue<'a>),
  Boolean(BooleanValue<'a>),
  Enum(EnumValue<'a>),
  List(ListValue<'a>),
  Object(ObjectValue<'a>)
}

/// BooleanValue : one of `true` `false`
pub struct BooleanValue<'a> {
  pub loc: Option<Location<'a>>,
  pub value: bool
}

impl_node_for! { BooleanValue }

/// EnumValue : Name but not `true`, `false` or `null`
pub struct EnumValue<'a> {
  pub loc: Option<Location<'a>>,
  pub name: Name<'a>
}

impl_node_for! { EnumValue }

/// ListValue[Const] :
///   - [ ]
///   - [ Value[?Const]+ ]
pub struct ListValue<'a> {
  pub loc: Option<Location<'a>>,
  pub values: Vec<Value<'a>>
}

impl_node_for! { ListValue }

/// ObjectValue[Const] :
///   - { }
///   - { ObjectField[?Const]+ }
pub struct ObjectValue<'a> {
  pub loc: Option<Location<'a>>,
  pub fields: Vec<ObjectField<'a>>
}

impl_node_for! { ObjectValue }

/// ObjectField[Const] : Name : Value[?Const]
pub struct ObjectField<'a> {
  pub loc: Option<Location<'a>>,
  pub name: Name<'a>,
  pub value: Value<'a>
}

impl_node_for! { ObjectField }

/// VariableDefinitions : ( VariableDefinition+ )
pub type VariableDefinitions<'a> = Vec<VariableDefinition<'a>>;

/// VariableDefinition : Variable : Type DefaultValue?
pub struct VariableDefinition<'a> {
  pub loc: Option<Location<'a>>,
  pub variable: Variable<'a>,
  pub type_: Type<'a>,
  pub default_value: Option<DefaultValue<'a>>
}

impl_node_for! { VariableDefinition }

/// Variable : $ Name
pub struct Variable<'a> {
  pub loc: Option<Location<'a>>,
  pub name: Name<'a>
}

impl_node_for! { Variable }

/// DefaultValue : = Value[Const]
pub type DefaultValue<'a> = Value<'a>;

/// Type :
///   - NamedType
///   - ListType
///   - NonNullType
pub enum Type<'a> {
  Named(NamedType<'a>),
  List(Box<ListType<'a>>),
  NonNullNamed(Box<NonNullNamedType<'a>>),
  NonNullList(Box<NonNullListType<'a>>)
}

/// NamedType : Name
pub struct NamedType<'a> {
  pub loc: Option<Location<'a>>,
  pub name: Name<'a>
}

impl_node_for! { NamedType }

/// ListType : [ Type ]
pub struct ListType<'a> {
  pub loc: Option<Location<'a>>,
  pub type_: Type<'a>
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
pub struct NonNullNamedType<'a> {
  pub loc: Option<Location<'a>>,
  pub type_: NamedType<'a>
}

impl_node_for! { NonNullNamedType }

/// See documentation for the `NonNullNamedType` struct.
pub struct NonNullListType<'a> {
  pub loc: Option<Location<'a>>,
  pub type_: ListType<'a>
}

impl_node_for! { NonNullListType }

/// Directives : Directive+
pub type Directives<'a> = Vec<Directive<'a>>;

/// Directive : @ Name Arguments?
pub struct Directive<'a> {
  pub loc: Option<Location<'a>>,
  pub name: Name<'a>,
  pub arguments: Option<Arguments<'a>>
}

impl_node_for! { Directive }
