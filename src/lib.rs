pub mod graphql;

#[cfg(test)]
mod tests {
  use super::graphql::*;
  use std::rc::Rc;

  #[test]
  fn test() {
    let INT = &Rc::new(GraphQLInt);
    let FLOAT = &Rc::new(GraphQLFloat);
    let STRING = &Rc::new(GraphQLString);
    let BOOLEAN = &Rc::new(GraphQLBoolean);

    let IMAGE = &GraphQLObjectBuilder::new("Image")
                   .with_description("Image Type")
                   .field("url", |f| f.type_of(STRING))
                   .field("width", |f| f.type_of(INT))
                   .field("height", |f| f.type_of(INT))
                   .build();

    let AUTHOR = &GraphQLObjectBuilder::new("Author")
                    .with_description("Author Type")
                    .field("id", |f| f.type_of(STRING))
                    .field("name", |f| f.type_of(STRING))
                    .field("pic", |f| {
                      f.type_of(IMAGE)
                       .arg("width", |a| a.type_of(INT))
                       .arg("height", |a| a.type_of(INT))
                    })
                    .field("recentArticle", |f| f.placeholder_type_of("Article"))
                    .build();

    let ARTICLE = &GraphQLObjectBuilder::new("Article")
                     .field("id", |f| f.type_of(STRING))
                     .field("isPublished", |f| f.type_of(BOOLEAN))
                     .field("author", |f| f.type_of(AUTHOR))
                     .field("title", |f| f.type_of(STRING))
                     .field("body", |f| f.type_of(STRING))
                     .build();

    AUTHOR.replace_field_placeholder_type("recentArticle", ARTICLE);
  }
}
