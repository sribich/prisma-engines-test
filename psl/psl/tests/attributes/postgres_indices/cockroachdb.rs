use psl::parser_database::IndexAlgorithm;

use crate::{Provider, common::*, with_header};

#[test]
fn array_field_default_ops() {
    let dml = indoc! {r#"
        model A {
          id Int   @id
          a  Int[]

          @@index([a], type: Gin)
        }
    "#};

    psl::parse_schema_without_extensions(with_header(dml, Provider::Postgres, &[]))
        .unwrap()
        .assert_has_model("A")
        .assert_index_on_fields(&["a"])
        .assert_type(IndexAlgorithm::Gin);
}

#[test]
fn no_ops_json_prisma_type() {
    let dml = indoc! {r#"
        model A {
          id Int  @id
          a  Json

          @@index([a], type: Gin)
        }
    "#};

    psl::parse_schema_without_extensions(with_header(dml, Provider::Postgres, &[]))
        .unwrap()
        .assert_has_model("A")
        .assert_index_on_fields(&["a"])
        .assert_type(IndexAlgorithm::Gin);
}

#[test]
fn with_raw_unsupported() {
    let dml = indoc! {r#"
        model A {
          id Int                     @id
          a  Unsupported("geometry")

          @@index([a], type: Gin)
        }
    "#};

    psl::parse_schema_without_extensions(with_header(dml, Provider::Postgres, &[]))
        .unwrap()
        .assert_has_model("A")
        .assert_index_on_fields(&["a"])
        .assert_type(IndexAlgorithm::Gin);
}

#[test]
fn jsonb_column_as_the_last_in_index() {
    let dml = indoc! {r#"
        model A {
          id Int  @id
          a  Json
          b  Int[]

          @@index([b, a], type: Gin)
        }
    "#};

    psl::parse_schema_without_extensions(with_header(dml, Provider::Postgres, &[]))
        .unwrap()
        .assert_has_model("A")
        .assert_index_on_fields(&["b", "a"])
        .assert_type(IndexAlgorithm::Gin);
}
