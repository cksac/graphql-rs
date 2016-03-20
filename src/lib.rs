extern crate graphql_parser;
pub use graphql_parser::*;

pub mod types;

#[cfg(test)]
mod tests {
  use types::*;
  use std::str::FromStr;

  // Custom Scalar type
  struct Int64;
  impl GraphQLType for Int64 {
    fn name(&self) -> &str {
      "Int64"
    }

    fn description(&self) -> Option<&str> {
      Some("The Int64 scalar type represents a signed 64‐bit numeric non‐fractional values.")
    }
  }

  impl GraphQLScalar for Int64 {
    type ValueType = i64;
    fn coerce_literal(&self, value: &str) -> Option<Self::ValueType> {
      i64::from_str(value).ok()
    }
  }

  #[test]
  fn test_scalar_type() {
    let int = GraphQLScalarType::int();
    assert_eq!("Int", int.name());
    assert_eq!(Some(10), int.coerce_literal("10"));
    assert_eq!(None, int.coerce_literal("10.1"));
    assert_eq!(None, int.coerce_literal("a"));
    assert_eq!(None, int.coerce_literal(&i64::max_value().to_string()));
    assert_eq!(None, int.coerce_literal(&i64::min_value().to_string()));

    let float = GraphQLScalarType::float();
    assert_eq!("Float", float.name());
    assert_eq!(Some(2.0), float.coerce_literal("2.0"));
    assert_eq!(Some(2.0), float.coerce_literal("2"));
    assert_eq!(None, float.coerce_literal("2.0a"));

    let string = GraphQLScalarType::string();
    assert_eq!("String", string.name());
    assert_eq!(Some(String::from("abc")), string.coerce_literal("abc"));
    assert_eq!(Some(String::from("2.0")), string.coerce_literal("2.0"));

    let boolean = GraphQLScalarType::boolean();
    assert_eq!("Boolean", boolean.name());
    assert_eq!(Some(true), boolean.coerce_literal("true"));
    assert_eq!(Some(false), boolean.coerce_literal("false"));
    assert_eq!(None, boolean.coerce_literal("1"));
    assert_eq!(None, boolean.coerce_literal("0"));
    assert_eq!(None, boolean.coerce_literal("True"));
    assert_eq!(None, boolean.coerce_literal("False"));
    assert_eq!(None, boolean.coerce_literal("TRUE"));
    assert_eq!(None, boolean.coerce_literal("FALSE"));

    let int64 = GraphQLScalarType::custom(|| Int64);
    assert_eq!("Int64", int64.name());
  }

  #[test]
  fn test_object_type() {
    let int = &GraphQLScalarType::int();
    let string = &GraphQLScalarType::string();
    let boolean = &GraphQLScalarType::boolean();
    let int64 = &GraphQLScalarType::custom(|| Int64);

    let image = &GraphQLObjectType::new("Image")
                   .description("Image Type")
                   .field("url", |f| f.type_of(string))
                   .field("width", |f| f.type_of(int))
                   .field("height", |f| f.type_of(int))
                   .build();
    assert_eq!("Image", image.name());

    let author = &GraphQLObjectType::new("Author")
                    .description("Author Type")
                    .field("id", |f| f.type_of(int64))
                    .field("name", |f| f.type_of(string))
                    .field("pic", |f| {
                      f.type_of(image)
                       .arg("width", |a| a.type_of(int))
                       .arg("height", |a| a.type_of(int))
                    })
                    .field("recentArticle", |f| f.placeholder_type_of("Article"))
                    .build();
    assert_eq!("Author", author.name());

    let article = &GraphQLObjectType::new("Article")
                     .field("id", |f| f.type_of(string))
                     .field("isPublished", |f| f.type_of(boolean))
                     .field("author", |f| f.type_of(author))
                     .field("title", |f| f.type_of(string))
                     .field("body", |f| f.type_of(string))
                     .build();
    assert_eq!("Article", article.name());

    author.replace_field_placeholder_type("recentArticle", article);
  }

  #[test]
  fn test_interface_type() {
    let int = &GraphQLScalarType::int();
    let string = &GraphQLScalarType::string();

    let named_entity = &GraphQLInterfaceType::new("NamedEntity")
                          .field("name", |f| f.type_of(string))
                          .build();
    assert_eq!("NamedEntity", named_entity.name());

    let person = &GraphQLObjectType::new("Person")
                    .field("name", |f| f.type_of(string))
                    .field("age", |f| f.type_of(int))
                    .impl_interface(named_entity)
                    .build();
    assert_eq!("Person", person.name());

    let business = &GraphQLObjectType::new("Business")
                      .field("name", |f| f.type_of(string))
                      .field("employeeCount", |f| f.type_of(int))
                      .impl_interface(named_entity)
                      .build();
    assert_eq!("Business", business.name());
  }

  #[test]
  fn test_union_type() {
    let int = &GraphQLScalarType::int();
    let string = &GraphQLScalarType::string();

    let person = &GraphQLObjectType::new("Person")
                    .field("name", |f| f.type_of(string))
                    .field("age", |f| f.type_of(int))
                    .build();
    assert_eq!("Person", person.name());

    let photo = &GraphQLObjectType::new("Photo")
                   .field("height", |f| f.type_of(int))
                   .field("width", |f| f.type_of(int))
                   .build();
    assert_eq!("Photo", photo.name());

    let search_result = &GraphQLUnionType::new("SearchResult")
                           .description("Result will be either Person or Photo")
                           .maybe_type_of(person)
                           .maybe_type_of(photo)
                           .build();

    assert_eq!("SearchResult", search_result.name());
  }

  #[test]
  fn test_enum_type() {
    let rgb = GraphQLEnumType::new("RGB")
                .value("RED", |v| v)
                .value("GREEN", |v| v)
                .value("BLUE", |v| v)
                .build();
    assert_eq!("RGB", rgb.name());

    let days = GraphQLEnumType::new("DAYS")
                 .description("Days of the week")
                 .value("SAT", |v| v.description("Satarday"))
                 .value("SUN", |v| v.description("Sunday"))
                 .value("MON", |v| v.description("Monday"))
                 .value("TUE", |v| v.description("Tuesday"))
                 .value("WED", |v| v.description("Wedsday"))
                 .value("THU", |v| v.description("Thusday"))
                 .value("FRI", |v| v.description("Friday"))
                 .build();
    assert_eq!("DAYS", days.name());
  }

  #[test]
  fn test_input_object_type() {
    let float = &GraphQLScalarType::float();

    let geo_point = &GraphQLInputObjectType::new("GeoPoint")
                       .field("lat", |f| f.type_of(float))
                       .field("lon", |f| f.type_of(float))
                       .field("alt", |f| f.type_of(float))
                       .build();
    assert_eq!("GeoPoint", geo_point.name());
  }
}
