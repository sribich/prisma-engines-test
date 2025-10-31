use crate::{Provider, common::*, with_header};

#[test]
fn sqlite_disallows_compound_unique_length_prefix() {
    let dml = indoc! {r#"
        model A {
          a String
          b String
          @@unique([a(length: 10), b(length: 30)])
        }
    "#};

    let dml = with_header(dml, Provider::Sqlite, &[]);
    let error = parse_unwrap_err(&dml);

    let expectation = expect![[r#"
        [1;91merror[0m: [1mError parsing attribute "@@unique": The length argument is not supported in an index definition with the current connector[0m
          [1;94m-->[0m  [4mschema.prisma:14[0m
        [1;94m   | [0m
        [1;94m13 | [0m  b String
        [1;94m14 | [0m  [1;91m@@unique([a(length: 10), b(length: 30)])[0m
        [1;94m   | [0m
    "#]];

    expectation.assert_eq(&error)
}

#[test]
fn sqlite_disallows_index_length_prefix() {
    let dml = indoc! {r#"
        model A {
          id Int @id
          a String

          @@index([a(length: 10)])
        }
    "#};

    let dml = with_header(dml, Provider::Sqlite, &[]);
    let error = parse_unwrap_err(&dml);

    let expectation = expect![[r#"
        [1;91merror[0m: [1mError parsing attribute "@@index": The length argument is not supported in an index definition with the current connector[0m
          [1;94m-->[0m  [4mschema.prisma:15[0m
        [1;94m   | [0m
        [1;94m14 | [0m
        [1;94m15 | [0m  [1;91m@@index([a(length: 10)])[0m
        [1;94m   | [0m
    "#]];

    expectation.assert_eq(&error)
}

#[test]
fn length_argument_does_not_work_with_decimal() {
    let dml = indoc! {r#"
        model A {
          id Int @id
          a Decimal

          @@index([a(length: 10)])
        }
    "#};

    let schema = with_header(dml, Provider::Mysql, &[]);
    let error = parse_unwrap_err(&schema);

    let expectation = expect![[r#"
        [1;91merror[0m: [1mError parsing attribute "@@index": The length argument is only allowed with field types `String` or `Bytes`.[0m
          [1;94m-->[0m  [4mschema.prisma:15[0m
        [1;94m   | [0m
        [1;94m14 | [0m
        [1;94m15 | [0m  [1;91m@@index([a(length: 10)])[0m
        [1;94m   | [0m
    "#]];

    expectation.assert_eq(&error)
}

#[test]
fn length_argument_does_not_work_with_json() {
    let dml = indoc! {r#"
        model A {
          id Int @id
          a Json

          @@index([a(length: 10)])
        }
    "#};

    let schema = with_header(dml, Provider::Mysql, &[]);
    let error = parse_unwrap_err(&schema);

    let expectation = expect![[r#"
        [1;91merror[0m: [1mError parsing attribute "@@index": The length argument is only allowed with field types `String` or `Bytes`.[0m
          [1;94m-->[0m  [4mschema.prisma:15[0m
        [1;94m   | [0m
        [1;94m14 | [0m
        [1;94m15 | [0m  [1;91m@@index([a(length: 10)])[0m
        [1;94m   | [0m
    "#]];

    expectation.assert_eq(&error)
}

#[test]
fn length_argument_does_not_work_with_datetime() {
    let dml = indoc! {r#"
        model A {
          id Int @id
          a DateTime

          @@index([a(length: 10)])
        }
    "#};

    let schema = with_header(dml, Provider::Mysql, &[]);
    let error = parse_unwrap_err(&schema);

    let expectation = expect![[r#"
        [1;91merror[0m: [1mError parsing attribute "@@index": The length argument is only allowed with field types `String` or `Bytes`.[0m
          [1;94m-->[0m  [4mschema.prisma:15[0m
        [1;94m   | [0m
        [1;94m14 | [0m
        [1;94m15 | [0m  [1;91m@@index([a(length: 10)])[0m
        [1;94m   | [0m
    "#]];

    expectation.assert_eq(&error)
}

#[test]
fn length_argument_does_not_work_with_boolean() {
    let dml = indoc! {r#"
        model A {
          id Int @id
          a Boolean

          @@index([a(length: 10)])
        }
    "#};

    let schema = with_header(dml, Provider::Mysql, &[]);
    let error = parse_unwrap_err(&schema);

    let expectation = expect![[r#"
        [1;91merror[0m: [1mError parsing attribute "@@index": The length argument is only allowed with field types `String` or `Bytes`.[0m
          [1;94m-->[0m  [4mschema.prisma:15[0m
        [1;94m   | [0m
        [1;94m14 | [0m
        [1;94m15 | [0m  [1;91m@@index([a(length: 10)])[0m
        [1;94m   | [0m
    "#]];

    expectation.assert_eq(&error)
}

#[test]
fn length_argument_does_not_work_with_float() {
    let dml = indoc! {r#"
        model A {
          id Int @id
          a Float

          @@index([a(length: 10)])
        }
    "#};

    let schema = with_header(dml, Provider::Mysql, &[]);
    let error = parse_unwrap_err(&schema);

    let expectation = expect![[r#"
        [1;91merror[0m: [1mError parsing attribute "@@index": The length argument is only allowed with field types `String` or `Bytes`.[0m
          [1;94m-->[0m  [4mschema.prisma:15[0m
        [1;94m   | [0m
        [1;94m14 | [0m
        [1;94m15 | [0m  [1;91m@@index([a(length: 10)])[0m
        [1;94m   | [0m
    "#]];

    expectation.assert_eq(&error)
}

#[test]
fn length_argument_does_not_work_with_bigint() {
    let dml = indoc! {r#"
        model A {
          id Int @id
          a BigInt

          @@index([a(length: 10)])
        }
    "#};

    let schema = with_header(dml, Provider::Mysql, &[]);
    let error = parse_unwrap_err(&schema);

    let expectation = expect![[r#"
        [1;91merror[0m: [1mError parsing attribute "@@index": The length argument is only allowed with field types `String` or `Bytes`.[0m
          [1;94m-->[0m  [4mschema.prisma:15[0m
        [1;94m   | [0m
        [1;94m14 | [0m
        [1;94m15 | [0m  [1;91m@@index([a(length: 10)])[0m
        [1;94m   | [0m
    "#]];

    expectation.assert_eq(&error)
}

#[test]
fn length_argument_does_not_work_with_int() {
    let dml = indoc! {r#"
        model A {
          id Int @id
          a Int

          @@index([a(length: 10)])
        }
    "#};

    let schema = with_header(dml, Provider::Mysql, &[]);
    let error = parse_unwrap_err(&schema);

    let expectation = expect![[r#"
        [1;91merror[0m: [1mError parsing attribute "@@index": The length argument is only allowed with field types `String` or `Bytes`.[0m
          [1;94m-->[0m  [4mschema.prisma:15[0m
        [1;94m   | [0m
        [1;94m14 | [0m
        [1;94m15 | [0m  [1;91m@@index([a(length: 10)])[0m
        [1;94m   | [0m
    "#]];

    expectation.assert_eq(&error)
}

#[test]
fn hash_index_doesnt_allow_sorting() {
    let dml = indoc! {r#"
        model A {
          id Int @id
          a  Int

          @@index([a(sort: Desc)], type: Hash)
        }
    "#};

    let schema = with_header(dml, Provider::Postgres, &[]);
    let error = parse_unwrap_err(&schema);

    let expectation = expect![[r#"
        [1;91merror[0m: [1mError parsing attribute "@@index": Hash type does not support sort option.[0m
          [1;94m-->[0m  [4mschema.prisma:15[0m
        [1;94m   | [0m
        [1;94m14 | [0m
        [1;94m15 | [0m  [1;91m@@index([a(sort: Desc)], type: Hash)[0m
        [1;94m   | [0m
    "#]];

    expectation.assert_eq(&error)
}

#[test]
fn hash_index_doesnt_work_on_mysql() {
    let dml = indoc! {r#"
        model A {
          id Int @id
          a  Int

          @@index([a], type: Hash)
        }
    "#};

    let schema = with_header(dml, Provider::Mysql, &[]);
    let error = parse_unwrap_err(&schema);

    let expectation = expect![[r#"
        [1;91merror[0m: [1mError parsing attribute "@@index": The given index type is not supported with the current connector[0m
          [1;94m-->[0m  [4mschema.prisma:15[0m
        [1;94m   | [0m
        [1;94m14 | [0m
        [1;94m15 | [0m  @@index([a], [1;91mtype: Hash[0m)
        [1;94m   | [0m
    "#]];

    expectation.assert_eq(&error)
}

#[test]
fn fulltext_index_length_attribute() {
    let dml = indoc! {r#"
        model A {
          id Int @id
          a String
          b String

          @@fulltext([a(length: 30), b])
        }
    "#};

    let schema = with_header(dml, Provider::Mysql, &[]);
    let error = parse_unwrap_err(&schema);

    let expectation = expect![[r#"
        [1;91merror[0m: [1mError parsing attribute "@@fulltext": The length argument is not supported in a @@fulltext attribute.[0m
          [1;94m-->[0m  [4mschema.prisma:16[0m
        [1;94m   | [0m
        [1;94m15 | [0m
        [1;94m16 | [0m  [1;91m@@fulltext([a(length: 30), b])[0m
        [1;94m   | [0m
    "#]];

    expectation.assert_eq(&error)
}

#[test]
fn hash_index_doesnt_work_on_sqlite() {
    let dml = indoc! {r#"
        model A {
          id Int @id
          a  Int

          @@index([a], type: Hash)
        }
    "#};

    let schema = with_header(dml, Provider::Sqlite, &[]);
    let error = parse_unwrap_err(&schema);

    let expectation = expect![[r#"
        [1;91merror[0m: [1mError parsing attribute "@@index": The given index type is not supported with the current connector[0m
          [1;94m-->[0m  [4mschema.prisma:15[0m
        [1;94m   | [0m
        [1;94m14 | [0m
        [1;94m15 | [0m  @@index([a], [1;91mtype: Hash[0m)
        [1;94m   | [0m
    "#]];

    expectation.assert_eq(&error)
}

#[test]
fn fulltext_index_sort_attribute() {
    let dml = indoc! {r#"
        model A {
          id Int @id
          a String
          b String

          @@fulltext([a(sort: Desc), b])
        }
    "#};

    let schema = with_header(dml, Provider::Mysql, &[]);
    let error = parse_unwrap_err(&schema);

    let expectation = expect![[r#"
        [1;91merror[0m: [1mError parsing attribute "@@fulltext": The sort argument is not supported in a @@fulltext attribute in the current connector.[0m
          [1;94m-->[0m  [4mschema.prisma:16[0m
        [1;94m   | [0m
        [1;94m15 | [0m
        [1;94m16 | [0m  [1;91m@@fulltext([a(sort: Desc), b])[0m
        [1;94m   | [0m
    "#]];

    expectation.assert_eq(&error)
}

#[test]
fn fulltext_index_postgres() {
    let dml = indoc! {r#"
        model A {
          id Int    @id
          a  String
          b  String

          @@fulltext([a, b])
        }
    "#};

    let schema = with_header(dml, Provider::Postgres, &[]);
    let error = parse_unwrap_err(&schema);

    let expectation = expect![[r#"
        [1;91merror[0m: [1mError parsing attribute "@@fulltext": Defining fulltext indexes is not supported with the current connector.[0m
          [1;94m-->[0m  [4mschema.prisma:16[0m
        [1;94m   | [0m
        [1;94m15 | [0m
        [1;94m16 | [0m  [1;91m@@fulltext([a, b])[0m
        [1;94m   | [0m
    "#]];

    expectation.assert_eq(&error)
}

#[test]
fn fulltext_index_sqlite() {
    let dml = indoc! {r#"
        model A {
          id Int    @id
          a  String
          b  String

          @@fulltext([a, b])
        }
    "#};

    let schema = with_header(dml, Provider::Sqlite, &[]);
    let error = parse_unwrap_err(&schema);

    let expectation = expect![[r#"
        [1;91merror[0m: [1mError parsing attribute "@@fulltext": Defining fulltext indexes is not supported with the current connector.[0m
          [1;94m-->[0m  [4mschema.prisma:16[0m
        [1;94m   | [0m
        [1;94m15 | [0m
        [1;94m16 | [0m  [1;91m@@fulltext([a, b])[0m
        [1;94m   | [0m
    "#]];

    expectation.assert_eq(&error)
}

#[test]
fn index_without_fields_must_error() {
    let schema = r#"
        generator js {
          provider = "prisma-client"
        }

        datasource db {
          provider = "mysql"
          url      = env("DATABASE_URL")
        }

        model Fulltext {
          id      Int    @id
          title   String @db.VarChar(255)
          content String @db.Text

          @@fulltext(fields:[], map: "a")
          @@index(fields: [ ], map: "b")
          @@unique(fields: [])
        }
    "#;

    let expected = expect![[r#"
        [1;91merror[0m: [1mError parsing attribute "@@index": The list of fields in an index cannot be empty. Please specify at least one field.[0m
          [1;94m-->[0m  [4mschema.prisma:17[0m
        [1;94m   | [0m
        [1;94m16 | [0m          @@fulltext(fields:[], map: "a")
        [1;94m17 | [0m          [1;91m@@index(fields: [ ], map: "b")[0m
        [1;94m   | [0m
        [1;91merror[0m: [1mError parsing attribute "@@unique": The list of fields in an index cannot be empty. Please specify at least one field.[0m
          [1;94m-->[0m  [4mschema.prisma:18[0m
        [1;94m   | [0m
        [1;94m17 | [0m          @@index(fields: [ ], map: "b")
        [1;94m18 | [0m          [1;91m@@unique(fields: [])[0m
        [1;94m   | [0m
        [1;91merror[0m: [1mError parsing attribute "@@fulltext": The list of fields in an index cannot be empty. Please specify at least one field.[0m
          [1;94m-->[0m  [4mschema.prisma:16[0m
        [1;94m   | [0m
        [1;94m15 | [0m
        [1;94m16 | [0m          [1;91m@@fulltext(fields:[], map: "a")[0m
        [1;94m   | [0m
    "#]];

    let error = parse_unwrap_err(schema);
    expected.assert_eq(&error);
}
