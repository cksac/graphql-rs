use super::*;

/// Definition :
///   - OperationDefinition
///   - FragmentDefinition
pub enum Definition<'a> {
  Operation(OperationDefinition<'a>),
  Fragment(FragmentDefinition<'a>),
}

/// OperationDefinition :
///   - SelectionSet
///   - OperationType Name? VariableDefinitions? Directives? SelectionSet
pub struct OperationDefinition<'a> {
  pub loc: Option<Location>,
  pub operation: OperationType,
  pub name: Option<Name<'a>>,
  pub variable_definitions: Option<VariableDefinitions<'a>>,
  pub directives: Option<Directives<'a>>,
  pub selection_set: SelectionSet<'a>,
}

impl_life_node_for! { OperationDefinition }

/// FragmentDefinition : fragment FragmentName TypeCondition Directives? SelectionSet
pub struct FragmentDefinition<'a> {
  pub loc: Option<Location>,
  pub name: Name<'a>,
  pub type_condition: TypeCondition<'a>,
  pub directives: Option<Directives<'a>>,
  pub selection_set: SelectionSet<'a>,
}

impl_life_node_for! { FragmentDefinition }

/// OperationType : one of query mutation
pub enum OperationType {
  Query,
  Mutation,
}

/// VariableDefinitions : ( VariableDefinition+ )
pub type VariableDefinitions<'a> = Vec<VariableDefinition<'a>>;

/// VariableDefinition : Variable : Type DefaultValue?
pub struct VariableDefinition<'a> {
  pub loc: Option<Location>,
  pub variable: Variable<'a>,
  pub type_: Type<'a>,
  pub default_value: Option<DefaultValue<'a>>,
}

impl_life_node_for! { VariableDefinition }
