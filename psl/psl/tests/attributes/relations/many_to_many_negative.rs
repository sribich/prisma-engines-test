use crate::{Provider, common::*, with_header};

#[test]
fn implicit_many_to_many_relation_fields_with_referential_actions() {
    let schema = indoc! {r#"
        datasource db {
          provider = "sqlite"
          url      = "file:./dev.db"
        }

        model Track {
          id        String     @id
          title     String
          playlists Playlist[] @relation(onDelete: Restrict, onUpdate: Restrict)
        }

        model Playlist {
          id     String  @id
          name   String
          tracks Track[] @relation(onDelete: Restrict, onUpdate: Restrict)
        }
    "#};

    let expect = expect![[r#"
        [1;91merror[0m: [1mError validating: Referential actions on implicit many-to-many relations are not supported[0m
          [1;94m-->[0m  [4mschema.prisma:15[0m
        [1;94m   | [0m
        [1;94m14 | [0m  name   String
        [1;94m15 | [0m  tracks Track[] @relation(onDelete: [1;91mRestrict[0m, onUpdate: Restrict)
        [1;94m   | [0m
        [1;91merror[0m: [1mError validating: Referential actions on implicit many-to-many relations are not supported[0m
          [1;94m-->[0m  [4mschema.prisma:15[0m
        [1;94m   | [0m
        [1;94m14 | [0m  name   String
        [1;94m15 | [0m  tracks Track[] @relation(onDelete: Restrict, onUpdate: [1;91mRestrict[0m)
        [1;94m   | [0m
        [1;91merror[0m: [1mError validating: Referential actions on implicit many-to-many relations are not supported[0m
          [1;94m-->[0m  [4mschema.prisma:9[0m
        [1;94m   | [0m
        [1;94m 8 | [0m  title     String
        [1;94m 9 | [0m  playlists Playlist[] @relation(onDelete: [1;91mRestrict[0m, onUpdate: Restrict)
        [1;94m   | [0m
        [1;91merror[0m: [1mError validating: Referential actions on implicit many-to-many relations are not supported[0m
          [1;94m-->[0m  [4mschema.prisma:9[0m
        [1;94m   | [0m
        [1;94m 8 | [0m  title     String
        [1;94m 9 | [0m  playlists Playlist[] @relation(onDelete: Restrict, onUpdate: [1;91mRestrict[0m)
        [1;94m   | [0m
    "#]];

    expect.assert_eq(&parse_unwrap_err(schema));
}

#[test]
fn embedded_many_to_many_relation_fields_with_referential_actions_postgres() {
    let dml = indoc! {r#"
        model A {
          id    Int   @id @map("_id")
          b_ids Int[]
          bs    B[]   @relation(fields: [b_ids], references: [id], onDelete: Restrict, onUpdate: Restrict)
        }

        model B {
          id    Int   @id @map("_id")
          a_ids Int[]
          as    A[]   @relation(fields: [a_ids], references: [id], onDelete: Restrict, onUpdate: Restrict)
        }
    "#};

    let expect = expect![[r#"
        [1;91merror[0m: [1mError validating: Embedded many-to-many relations are not supported on Postgres. Please use the syntax defined in https://pris.ly/d/relational-database-many-to-many[0m
          [1;94m-->[0m  [4mschema.prisma:14[0m
        [1;94m   | [0m
        [1;94m13 | [0m  b_ids Int[]
        [1;94m14 | [0m  [1;91mbs    B[]   @relation(fields: [b_ids], references: [id], onDelete: Restrict, onUpdate: Restrict)[0m
        [1;94m15 | [0m}
        [1;94m   | [0m
        [1;91merror[0m: [1mError validating: Embedded many-to-many relations are not supported on Postgres. Please use the syntax defined in https://pris.ly/d/relational-database-many-to-many[0m
          [1;94m-->[0m  [4mschema.prisma:20[0m
        [1;94m   | [0m
        [1;94m19 | [0m  a_ids Int[]
        [1;94m20 | [0m  [1;91mas    A[]   @relation(fields: [a_ids], references: [id], onDelete: Restrict, onUpdate: Restrict)[0m
        [1;94m21 | [0m}
        [1;94m   | [0m
    "#]];

    let dml = with_header(dml, Provider::Postgres, &[]);
    expect.assert_eq(&parse_unwrap_err(&dml));
}

#[test]
fn embedded_many_to_many_must_define_references_on_both_sides_postgres() {
    let dml = indoc! {r#"
        model A {
          id    Int   @id @map("_id")
          b_ids Int[]
          bs    B[]   @relation(fields: [b_ids], references: [id])
        }

        model B {
          id    Int   @id @map("_id")
          a_ids Int[]
          as    A[]   @relation(fields: [a_ids])
        }
    "#};

    let expect = expect![[r#"
        [1;91merror[0m: [1mError validating: Embedded many-to-many relations are not supported on Postgres. Please use the syntax defined in https://pris.ly/d/relational-database-many-to-many[0m
          [1;94m-->[0m  [4mschema.prisma:14[0m
        [1;94m   | [0m
        [1;94m13 | [0m  b_ids Int[]
        [1;94m14 | [0m  [1;91mbs    B[]   @relation(fields: [b_ids], references: [id])[0m
        [1;94m15 | [0m}
        [1;94m   | [0m
        [1;91merror[0m: [1mError validating: Embedded many-to-many relations are not supported on Postgres. Please use the syntax defined in https://pris.ly/d/relational-database-many-to-many[0m
          [1;94m-->[0m  [4mschema.prisma:20[0m
        [1;94m   | [0m
        [1;94m19 | [0m  a_ids Int[]
        [1;94m20 | [0m  [1;91mas    A[]   @relation(fields: [a_ids])[0m
        [1;94m21 | [0m}
        [1;94m   | [0m
    "#]];

    let dml = with_header(dml, Provider::Postgres, &[]);
    expect.assert_eq(&parse_unwrap_err(&dml));

    let dml = indoc! {r#"
        model A {
          id    Int   @id @map("_id")
          b_ids Int[]
          bs    B[]   @relation(fields: [b_ids])
        }

        model B {
          id    Int   @id @map("_id")
          a_ids Int[]
          as    A[]   @relation(fields: [a_ids], references: [id])
        }
    "#};

    let expect = expect![[r#"
        [1;91merror[0m: [1mError validating: Embedded many-to-many relations are not supported on Postgres. Please use the syntax defined in https://pris.ly/d/relational-database-many-to-many[0m
          [1;94m-->[0m  [4mschema.prisma:14[0m
        [1;94m   | [0m
        [1;94m13 | [0m  b_ids Int[]
        [1;94m14 | [0m  [1;91mbs    B[]   @relation(fields: [b_ids])[0m
        [1;94m15 | [0m}
        [1;94m   | [0m
        [1;91merror[0m: [1mError validating: Embedded many-to-many relations are not supported on Postgres. Please use the syntax defined in https://pris.ly/d/relational-database-many-to-many[0m
          [1;94m-->[0m  [4mschema.prisma:20[0m
        [1;94m   | [0m
        [1;94m19 | [0m  a_ids Int[]
        [1;94m20 | [0m  [1;91mas    A[]   @relation(fields: [a_ids], references: [id])[0m
        [1;94m21 | [0m}
        [1;94m   | [0m
    "#]];

    let dml = with_header(dml, Provider::Postgres, &[]);
    expect.assert_eq(&parse_unwrap_err(&dml));

    let dml = indoc! {r#"
        model A {
          id    Int   @id @map("_id")
          b_ids Int[]
          bs    B[]   @relation(fields: [b_ids])
        }

        model B {
          id    Int   @id @map("_id")
          a_ids Int[]
          as    A[]   @relation(fields: [a_ids])
        }
    "#};

    let expect = expect![[r#"
        [1;91merror[0m: [1mError validating: Embedded many-to-many relations are not supported on Postgres. Please use the syntax defined in https://pris.ly/d/relational-database-many-to-many[0m
          [1;94m-->[0m  [4mschema.prisma:14[0m
        [1;94m   | [0m
        [1;94m13 | [0m  b_ids Int[]
        [1;94m14 | [0m  [1;91mbs    B[]   @relation(fields: [b_ids])[0m
        [1;94m15 | [0m}
        [1;94m   | [0m
        [1;91merror[0m: [1mError validating: Embedded many-to-many relations are not supported on Postgres. Please use the syntax defined in https://pris.ly/d/relational-database-many-to-many[0m
          [1;94m-->[0m  [4mschema.prisma:20[0m
        [1;94m   | [0m
        [1;94m19 | [0m  a_ids Int[]
        [1;94m20 | [0m  [1;91mas    A[]   @relation(fields: [a_ids])[0m
        [1;94m21 | [0m}
        [1;94m   | [0m
    "#]];

    let dml = with_header(dml, Provider::Postgres, &[]);
    expect.assert_eq(&parse_unwrap_err(&dml));
}

#[test]
fn embedded_many_to_many_must_define_fields_on_both_sides_postgres() {
    let dml = indoc! {r#"
        model A {
          id    Int   @id @map("_id")
          b_ids Int[]
          bs    B[]   @relation(fields: [b_ids], references: [id])
        }

        model B {
          id    Int   @id @map("_id")
          a_ids Int[]
          as    A[]   @relation(references: [id])
        }
    "#};

    let expect = expect![[r#"
        [1;91merror[0m: [1mError validating: Embedded many-to-many relations are not supported on Postgres. Please use the syntax defined in https://pris.ly/d/relational-database-many-to-many[0m
          [1;94m-->[0m  [4mschema.prisma:14[0m
        [1;94m   | [0m
        [1;94m13 | [0m  b_ids Int[]
        [1;94m14 | [0m  [1;91mbs    B[]   @relation(fields: [b_ids], references: [id])[0m
        [1;94m15 | [0m}
        [1;94m   | [0m
        [1;91merror[0m: [1mError validating: Embedded many-to-many relations are not supported on Postgres. Please use the syntax defined in https://pris.ly/d/relational-database-many-to-many[0m
          [1;94m-->[0m  [4mschema.prisma:20[0m
        [1;94m   | [0m
        [1;94m19 | [0m  a_ids Int[]
        [1;94m20 | [0m  [1;91mas    A[]   @relation(references: [id])[0m
        [1;94m21 | [0m}
        [1;94m   | [0m
    "#]];

    let dml = with_header(dml, Provider::Postgres, &[]);
    expect.assert_eq(&parse_unwrap_err(&dml));

    let dml = indoc! {r#"
        model A {
          id    Int   @id @map("_id")
          b_ids Int[]
          bs    B[]   @relation(references: [id])
        }

        model B {
          id    Int   @id @map("_id")
          a_ids Int[]
          as    A[]   @relation(fields: [a_ids], references: [id])
        }
    "#};

    let expect = expect![[r#"
        [1;91merror[0m: [1mError validating: Embedded many-to-many relations are not supported on Postgres. Please use the syntax defined in https://pris.ly/d/relational-database-many-to-many[0m
          [1;94m-->[0m  [4mschema.prisma:14[0m
        [1;94m   | [0m
        [1;94m13 | [0m  b_ids Int[]
        [1;94m14 | [0m  [1;91mbs    B[]   @relation(references: [id])[0m
        [1;94m15 | [0m}
        [1;94m   | [0m
        [1;91merror[0m: [1mError validating: Embedded many-to-many relations are not supported on Postgres. Please use the syntax defined in https://pris.ly/d/relational-database-many-to-many[0m
          [1;94m-->[0m  [4mschema.prisma:20[0m
        [1;94m   | [0m
        [1;94m19 | [0m  a_ids Int[]
        [1;94m20 | [0m  [1;91mas    A[]   @relation(fields: [a_ids], references: [id])[0m
        [1;94m21 | [0m}
        [1;94m   | [0m
    "#]];

    let dml = with_header(dml, Provider::Postgres, &[]);
    expect.assert_eq(&parse_unwrap_err(&dml));
}

#[test]
fn embedded_many_to_many_relations_do_not_work_on_postgresql() {
    let dml = indoc! {r#"
        model A {
          id    Int      @id
          b_ids Int[]
          bs    B[]      @relation(fields: [b_ids], references: [id])
        }

        model B {
          id    Int      @id
          a_ids Int[]
          as    A[]      @relation(fields: [a_ids], references: [id])
        }
    "#};

    let expect = expect![[r#"
        [1;91merror[0m: [1mError validating: Embedded many-to-many relations are not supported on Postgres. Please use the syntax defined in https://pris.ly/d/relational-database-many-to-many[0m
          [1;94m-->[0m  [4mschema.prisma:14[0m
        [1;94m   | [0m
        [1;94m13 | [0m  b_ids Int[]
        [1;94m14 | [0m  [1;91mbs    B[]      @relation(fields: [b_ids], references: [id])[0m
        [1;94m15 | [0m}
        [1;94m   | [0m
        [1;91merror[0m: [1mError validating: Embedded many-to-many relations are not supported on Postgres. Please use the syntax defined in https://pris.ly/d/relational-database-many-to-many[0m
          [1;94m-->[0m  [4mschema.prisma:20[0m
        [1;94m   | [0m
        [1;94m19 | [0m  a_ids Int[]
        [1;94m20 | [0m  [1;91mas    A[]      @relation(fields: [a_ids], references: [id])[0m
        [1;94m21 | [0m}
        [1;94m   | [0m
    "#]];

    let dml = with_header(dml, Provider::Postgres, &[]);
    expect.assert_eq(&parse_unwrap_err(&dml));
}

#[test]
fn embedded_many_to_many_relations_must_refer_an_id_from_both_sides_postgres() {
    let dml = indoc! {r#"
        model A {
          id    Int   @id @map("_id")
          u1    Int   @unique
          b_ids Int[]
          bs    B[]   @relation(fields: [b_ids], references: [u2])
        }

        model B {
          id    Int   @id @map("_id")
          u2    Int   @unique
          a_ids Int[]
          as    A[]   @relation(fields: [a_ids], references: [u1])
        }
    "#};

    let expect = expect![[r#"
        [1;91merror[0m: [1mError validating: Embedded many-to-many relations are not supported on Postgres. Please use the syntax defined in https://pris.ly/d/relational-database-many-to-many[0m
          [1;94m-->[0m  [4mschema.prisma:15[0m
        [1;94m   | [0m
        [1;94m14 | [0m  b_ids Int[]
        [1;94m15 | [0m  [1;91mbs    B[]   @relation(fields: [b_ids], references: [u2])[0m
        [1;94m16 | [0m}
        [1;94m   | [0m
        [1;91merror[0m: [1mError validating: Embedded many-to-many relations are not supported on Postgres. Please use the syntax defined in https://pris.ly/d/relational-database-many-to-many[0m
          [1;94m-->[0m  [4mschema.prisma:22[0m
        [1;94m   | [0m
        [1;94m21 | [0m  a_ids Int[]
        [1;94m22 | [0m  [1;91mas    A[]   @relation(fields: [a_ids], references: [u1])[0m
        [1;94m23 | [0m}
        [1;94m   | [0m
    "#]];

    let dml = with_header(dml, Provider::Postgres, &[]);
    expect.assert_eq(&parse_unwrap_err(&dml));
}

#[test]
fn embedded_many_to_many_fields_must_be_an_array_of_correct_type_postgres() {
    let dml = indoc! {r#"
        model A {
          id    Int   @id @map("_id")
          b_ids Int[]
          bs    B[]   @relation(fields: [b_ids], references: [id])
        }

        model B {
          id    String @id @map("_id")
          a_ids Int[]
          as    A[]    @relation(fields: [a_ids], references: [id])
        }
    "#};

    let expect = expect![[r#"
        [1;91merror[0m: [1mError validating: Embedded many-to-many relations are not supported on Postgres. Please use the syntax defined in https://pris.ly/d/relational-database-many-to-many[0m
          [1;94m-->[0m  [4mschema.prisma:14[0m
        [1;94m   | [0m
        [1;94m13 | [0m  b_ids Int[]
        [1;94m14 | [0m  [1;91mbs    B[]   @relation(fields: [b_ids], references: [id])[0m
        [1;94m15 | [0m}
        [1;94m   | [0m
        [1;91merror[0m: [1mError validating: Embedded many-to-many relations are not supported on Postgres. Please use the syntax defined in https://pris.ly/d/relational-database-many-to-many[0m
          [1;94m-->[0m  [4mschema.prisma:20[0m
        [1;94m   | [0m
        [1;94m19 | [0m  a_ids Int[]
        [1;94m20 | [0m  [1;91mas    A[]    @relation(fields: [a_ids], references: [id])[0m
        [1;94m21 | [0m}
        [1;94m   | [0m
    "#]];

    let dml = with_header(dml, Provider::Postgres, &[]);
    expect.assert_eq(&parse_unwrap_err(&dml));
}

#[test]
fn embedded_many_to_many_fields_must_be_an_array_of_correct_native_type_postgres() {
    let dml = indoc! {r#"
        model A {
          id    Int      @id @map("_id")
          b_ids String[] @test.VarChar(255)
          bs    B[]      @relation(fields: [b_ids], references: [id])
        }

        model B {
          id    String   @id @map("_id")
          a_ids String[] @test.VarChar(255)
          as    A[]      @relation(fields: [a_ids], references: [id])
        }
    "#};

    let expect = expect![[r#"
        [1;91merror[0m: [1mError validating: Embedded many-to-many relations are not supported on Postgres. Please use the syntax defined in https://pris.ly/d/relational-database-many-to-many[0m
          [1;94m-->[0m  [4mschema.prisma:14[0m
        [1;94m   | [0m
        [1;94m13 | [0m  b_ids String[] @test.VarChar(255)
        [1;94m14 | [0m  [1;91mbs    B[]      @relation(fields: [b_ids], references: [id])[0m
        [1;94m15 | [0m}
        [1;94m   | [0m
        [1;91merror[0m: [1mError validating: Embedded many-to-many relations are not supported on Postgres. Please use the syntax defined in https://pris.ly/d/relational-database-many-to-many[0m
          [1;94m-->[0m  [4mschema.prisma:20[0m
        [1;94m   | [0m
        [1;94m19 | [0m  a_ids String[] @test.VarChar(255)
        [1;94m20 | [0m  [1;91mas    A[]      @relation(fields: [a_ids], references: [id])[0m
        [1;94m21 | [0m}
        [1;94m   | [0m
    "#]];

    let dml = with_header(dml, Provider::Postgres, &[]);
    expect.assert_eq(&parse_unwrap_err(&dml));
}

#[test]
fn embedded_many_to_many_fields_must_be_an_array_postgres() {
    let dml = indoc! {r#"
        model A {
          id    Int    @id @map("_id")
          b_ids String
          bs    B[]    @relation(fields: [b_ids], references: [id])
        }

        model B {
          id    String @id @map("_id")
          a_ids Int[]
          as    A[]    @relation(fields: [a_ids], references: [id])
        }
    "#};

    let expect = expect![[r#"
        [1;91merror[0m: [1mError validating: Embedded many-to-many relations are not supported on Postgres. Please use the syntax defined in https://pris.ly/d/relational-database-many-to-many[0m
          [1;94m-->[0m  [4mschema.prisma:14[0m
        [1;94m   | [0m
        [1;94m13 | [0m  b_ids String
        [1;94m14 | [0m  [1;91mbs    B[]    @relation(fields: [b_ids], references: [id])[0m
        [1;94m15 | [0m}
        [1;94m   | [0m
        [1;91merror[0m: [1mError validating: Embedded many-to-many relations are not supported on Postgres. Please use the syntax defined in https://pris.ly/d/relational-database-many-to-many[0m
          [1;94m-->[0m  [4mschema.prisma:20[0m
        [1;94m   | [0m
        [1;94m19 | [0m  a_ids Int[]
        [1;94m20 | [0m  [1;91mas    A[]    @relation(fields: [a_ids], references: [id])[0m
        [1;94m21 | [0m}
        [1;94m   | [0m
    "#]];

    let dml = with_header(dml, Provider::Postgres, &[]);
    expect.assert_eq(&parse_unwrap_err(&dml));
}
