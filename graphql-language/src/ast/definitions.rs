use super::*;

/// Definition :
///   - OperationDefinition
///   - FragmentDefinition
pub enum Definition {
  Operation(OperationDefinition),
  Fragment(FragmentDefinition),
}

/// OperationDefinition :
///   - SelectionSet
///   - OperationType Name? VariableDefinitions? Directives? SelectionSet
pub struct OperationDefinition {
  pub loc: Option<Location>,
  pub operation: OperationType,
  pub name: Option<Name>,
  pub variable_definitions: Option<VariableDefinitions>,
  pub directives: Option<Directives>,
  pub selection_set: SelectionSet,
}

impl_node_for! { OperationDefinition }

/// FragmentDefinition : fragment FragmentName TypeCondition Directives? SelectionSet
pub struct FragmentDefinition {
  pub loc: Option<Location>,
  pub name: Name,
  pub type_condition: TypeCondition,
  pub directives: Option<Directives>,
  pub selection_set: SelectionSet,
}

impl_node_for! { FragmentDefinition }

/// OperationType : one of query mutation
pub enum OperationType {
  Query,
  Mutation,
}

/// VariableDefinitions : ( VariableDefinition+ )
pub type VariableDefinitions = Vec<VariableDefinition>;

/// VariableDefinition : Variable : Type DefaultValue?
pub struct VariableDefinition {
  pub loc: Option<Location>,
  pub variable: Variable,
  pub type_: Type,
  pub default_value: Option<DefaultValue>,
}

impl_node_for! { VariableDefinition }
