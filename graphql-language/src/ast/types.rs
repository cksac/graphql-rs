use super::*;

/// Type :
///   - NamedType
///   - ListType
///   - NonNullType
pub enum Type {
  Named(NamedType),
  List(Box<ListType>),
  NonNullNamed(NonNullNamedType),
  NonNullList(Box<NonNullListType>),
}

/// NamedType : Name
pub type NamedType = Name;

/// ListType : [ Type ]
pub struct ListType {
  pub loc: Option<Location>,
  pub type_: Type,
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
  pub loc: Option<Location>,
  pub type_: NamedType,
}

impl_node_for! { NonNullNamedType }

/// See documentation for the `NonNullNamedType` struct.
pub struct NonNullListType {
  pub loc: Option<Location>,
  pub type_: ListType,
}

impl_node_for! { NonNullListType }

/// TypeCondition : on NamedType
pub type TypeCondition = NamedType;
