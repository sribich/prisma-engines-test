use crate::common::*;

#[test]
fn map_attribute() {
    let dml = r#"
    model User {
        id Int @id
        firstName String @map("first_name")

        @@map("user")
    }

    model Post {
        id Int @id
        text String @map(name: "post_text")

        @@map(name: "posti")
    }
    "#;

    let schema = psl::parse_schema_without_extensions(dml).unwrap();

    let user = schema.assert_has_model("User");
    user.assert_mapped_name("user");
    user.assert_has_scalar_field("firstName")
        .assert_mapped_name("first_name");

    let post = schema.assert_has_model("Post");
    post.assert_mapped_name("posti");
    post.assert_has_scalar_field("text").assert_mapped_name("post_text");
}
