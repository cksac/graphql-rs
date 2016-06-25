use super::*;

/// SelectionSet : { Selection+ }
pub struct SelectionSet<'a> {
  pub loc: Option<Location>,
  pub selections: Vec<Selection<'a>>,
}

impl_life_node_for! { SelectionSet }

/// Selection :
///   - Field
///   - FragmentSpread
///   - InlineFragment
pub enum Selection<'a> {
  Field(Field<'a>),
  FragmentSpread(FragmentSpread<'a>),
  InlineFragment(InlineFragment<'a>),
}

/// FragmentSpread : ... FragmentName Directives?
pub struct FragmentSpread<'a> {
  pub loc: Option<Location>,
  pub name: Name<'a>,
  pub directives: Option<Directives<'a>>,
}

impl_life_node_for! { FragmentSpread }

/// InlineFragment : ... TypeCondition? Directives? SelectionSet
pub struct InlineFragment<'a> {
  pub loc: Option<Location>,
  pub type_condition: Option<TypeCondition<'a>>,
  pub directives: Option<Directives<'a>>,
  pub selection_set: SelectionSet<'a>,
}

impl_life_node_for! { InlineFragment }

/// FragmentName : Name but not `on`
pub type FragmentName<'a> = Name<'a>;

/// Field : Alias? Name Arguments? Directives? SelectionSet?
pub struct Field<'a> {
  pub loc: Option<Location>,
  pub alias: Option<Alias<'a>>,
  pub name: Name<'a>,
  pub arguments: Option<Arguments<'a>>,
  pub directives: Option<Directives<'a>>,
  pub selection_set: Option<SelectionSet<'a>>,
}

impl_life_node_for! { Field }
