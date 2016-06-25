use super::*;

/// SelectionSet : { Selection+ }
pub struct SelectionSet {
  pub loc: Option<Location>,
  pub selections: Vec<Selection>,
}

impl_node_for! { SelectionSet }

/// Selection :
///   - Field
///   - FragmentSpread
///   - InlineFragment
pub enum Selection {
  Field(Field),
  FragmentSpread(FragmentSpread),
  InlineFragment(InlineFragment),
}

/// FragmentSpread : ... FragmentName Directives?
pub struct FragmentSpread {
  pub loc: Option<Location>,
  pub name: Name,
  pub directives: Option<Directives>,
}

impl_node_for! { FragmentSpread }

/// InlineFragment : ... TypeCondition? Directives? SelectionSet
pub struct InlineFragment {
  pub loc: Option<Location>,
  pub type_condition: Option<TypeCondition>,
  pub directives: Option<Directives>,
  pub selection_set: SelectionSet,
}

impl_node_for! { InlineFragment }

/// FragmentName : Name but not `on`
pub type FragmentName = Name;

/// Field : Alias? Name Arguments? Directives? SelectionSet?
pub struct Field {
  pub loc: Option<Location>,
  pub alias: Option<Alias>,
  pub name: Name,
  pub arguments: Option<Arguments>,
  pub directives: Option<Directives>,
  pub selection_set: Option<SelectionSet>,
}

impl_node_for! { Field }
