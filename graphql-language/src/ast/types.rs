use super::*;

/// Type :
///   - NamedType
///   - ListType
///   - NonNullType
pub enum Type<'a> {
  Named(NamedType<'a>),
  List(Box<ListType<'a>>),
  NonNullNamed(Box<NonNullNamedType<'a>>),
  NonNullList(Box<NonNullListType<'a>>),
}

/// NamedType : Name
pub type NamedType<'a> = Name<'a>;

/// ListType : [ Type ]
pub struct ListType<'a> {
  pub loc: Option<Location>,
  pub type_: Type<'a>,
}

impl_life_node_for! { ListType }

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
  pub loc: Option<Location>,
  pub type_: NamedType<'a>,
}

impl_life_node_for! { NonNullNamedType }

/// See documentation for the `NonNullNamedType` struct.
pub struct NonNullListType<'a> {
  pub loc: Option<Location>,
  pub type_: ListType<'a>,
}

impl_life_node_for! { NonNullListType }

/// TypeCondition : on NamedType
pub type TypeCondition<'a> = NamedType<'a>;
