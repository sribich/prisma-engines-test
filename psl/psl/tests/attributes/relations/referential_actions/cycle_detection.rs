use crate::common::*;

#[test]
fn cycles_are_allowed_outside_of_emulation_and_sqlserver() {
    let dml = indoc! {
        r#"
        datasource db {
            provider = "mysql"
            url = "mysql://"
        }

        generator js1 {
          provider = "javascript"
        }

        model A {
            id     Int  @id @default(autoincrement())
            child  A?   @relation(name: "a_self_relation")
            parent A?   @relation(name: "a_self_relation", fields: [aId], references: [id], onDelete: Cascade)
            aId    Int? @unique
        }
    "#};

    assert_valid(dml)
}

#[test]
fn emulated_cascading_on_delete_self_relations() {
    let dml = indoc! {
        r#"
        datasource db {
            provider = "mysql"
            url = "mysql://"
            relationMode = "prisma"
        }

        generator js1 {
          provider = "javascript"
        }

        model A {
            id     Int  @id @default(autoincrement())
            child  A?   @relation(name: "a_self_relation")
            parent A?   @relation(name: "a_self_relation", fields: [aId], references: [id], onDelete: Cascade)
            aId    Int? @unique
        }
    "#};

    let expect = expect![[r#"
        [1;91merror[0m: [1mError validating: A self-relation must have `onDelete` and `onUpdate` referential actions set to `NoAction` in one of the @relation attributes. (Implicit default `onUpdate`: `Cascade`) Read more at https://pris.ly/d/cyclic-referential-actions[0m
          [1;94m-->[0m  [4mschema.prisma:14[0m
        [1;94m   | [0m
        [1;94m13 | [0m    child  A?   @relation(name: "a_self_relation")
        [1;94m14 | [0m    [1;91mparent A?   @relation(name: "a_self_relation", fields: [aId], references: [id], onDelete: Cascade)[0m
        [1;94m15 | [0m    aId    Int? @unique
        [1;94m   | [0m
    "#]];

    expect.assert_eq(&parse_unwrap_err(dml));
}

#[test]
fn emulated_cascading_on_update_self_relations() {
    let dml = indoc! {
        r#"
        datasource db {
            provider = "mysql"
            url = "mysql://"
            relationMode = "prisma"
        }

        generator js1 {
          provider = "javascript"
        }

        model A {
            id     Int  @id @default(autoincrement())
            child  A?   @relation(name: "a_self_relation")
            parent A?   @relation(name: "a_self_relation", fields: [aId], references: [id], onUpdate: Cascade)
            aId    Int? @unique
        }
    "#};

    let expect = expect![[r#"
        [1;91merror[0m: [1mError validating: A self-relation must have `onDelete` and `onUpdate` referential actions set to `NoAction` in one of the @relation attributes. (Implicit default `onDelete`: `SetNull`) Read more at https://pris.ly/d/cyclic-referential-actions[0m
          [1;94m-->[0m  [4mschema.prisma:14[0m
        [1;94m   | [0m
        [1;94m13 | [0m    child  A?   @relation(name: "a_self_relation")
        [1;94m14 | [0m    [1;91mparent A?   @relation(name: "a_self_relation", fields: [aId], references: [id], onUpdate: Cascade)[0m
        [1;94m15 | [0m    aId    Int? @unique
        [1;94m   | [0m
    "#]];

    expect.assert_eq(&parse_unwrap_err(dml));
}

#[test]
fn emulated_default_setting_on_delete_self_relations() {
    let dml = indoc! {
        r#"
        datasource db {
            provider = "mysql"
            url = "mysql://"
            relationMode = "prisma"
        }

        generator js1 {
          provider = "javascript"
        }

        model A {
            id     Int  @id @default(autoincrement())
            child  A?   @relation(name: "a_self_relation")
            parent A?   @relation(name: "a_self_relation", fields: [aId], references: [id], onDelete: SetDefault)
            aId    Int? @unique
        }
    "#};

    let expect = expect![[r#"
        [1;91merror[0m: [1mError validating: Invalid referential action: `SetDefault`. Allowed values: (`Cascade`, `Restrict`, `NoAction`, `SetNull`)[0m
          [1;94m-->[0m  [4mschema.prisma:14[0m
        [1;94m   | [0m
        [1;94m13 | [0m    child  A?   @relation(name: "a_self_relation")
        [1;94m14 | [0m    parent A?   @relation(name: "a_self_relation", fields: [aId], references: [id], [1;91monDelete: SetDefault[0m)
        [1;94m   | [0m
        [1;91merror[0m: [1mError validating: A self-relation must have `onDelete` and `onUpdate` referential actions set to `NoAction` in one of the @relation attributes. (Implicit default `onUpdate`: `Cascade`) Read more at https://pris.ly/d/cyclic-referential-actions[0m
          [1;94m-->[0m  [4mschema.prisma:14[0m
        [1;94m   | [0m
        [1;94m13 | [0m    child  A?   @relation(name: "a_self_relation")
        [1;94m14 | [0m    [1;91mparent A?   @relation(name: "a_self_relation", fields: [aId], references: [id], onDelete: SetDefault)[0m
        [1;94m15 | [0m    aId    Int? @unique
        [1;94m   | [0m
    "#]];

    expect.assert_eq(&parse_unwrap_err(dml));
}

#[test]
fn emulated_default_setting_on_update_self_relations() {
    let dml = indoc! {
        r#"
        datasource db {
            provider = "mysql"
            url = "mysql://"
            relationMode = "prisma"
        }

        generator js1 {
          provider = "javascript"
        }

        model A {
            id     Int  @id @default(autoincrement())
            child  A?   @relation(name: "a_self_relation")
            parent A?   @relation(name: "a_self_relation", fields: [aId], references: [id], onUpdate: SetDefault)
            aId    Int? @unique
        }
    "#};

    let expect = expect![[r#"
        [1;91merror[0m: [1mError validating: Invalid referential action: `SetDefault`. Allowed values: (`Cascade`, `Restrict`, `NoAction`, `SetNull`)[0m
          [1;94m-->[0m  [4mschema.prisma:14[0m
        [1;94m   | [0m
        [1;94m13 | [0m    child  A?   @relation(name: "a_self_relation")
        [1;94m14 | [0m    parent A?   @relation(name: "a_self_relation", fields: [aId], references: [id], [1;91monUpdate: SetDefault[0m)
        [1;94m   | [0m
        [1;91merror[0m: [1mError validating: A self-relation must have `onDelete` and `onUpdate` referential actions set to `NoAction` in one of the @relation attributes. (Implicit default `onDelete`: `SetNull`) Read more at https://pris.ly/d/cyclic-referential-actions[0m
          [1;94m-->[0m  [4mschema.prisma:14[0m
        [1;94m   | [0m
        [1;94m13 | [0m    child  A?   @relation(name: "a_self_relation")
        [1;94m14 | [0m    [1;91mparent A?   @relation(name: "a_self_relation", fields: [aId], references: [id], onUpdate: SetDefault)[0m
        [1;94m15 | [0m    aId    Int? @unique
        [1;94m   | [0m
    "#]];

    expect.assert_eq(&parse_unwrap_err(dml));
}

#[test]
fn emulated_cascading_cyclic_one_hop_relations() {
    let dml = indoc! {
        r#"
        datasource db {
            provider = "mysql"
            url = "mysql://"
            relationMode = "prisma"
        }

        generator js1 {
          provider = "javascript"
        }

        model A {
            id     Int  @id @default(autoincrement())
            b      B    @relation(name: "foo", fields: [bId], references: [id], onDelete: Cascade)
            bId    Int
            bs     B[]  @relation(name: "bar")
        }

        model B {
            id     Int @id @default(autoincrement())
            a      A   @relation(name: "bar", fields: [aId], references: [id], onUpdate: Cascade)
            as     A[] @relation(name: "foo")
            aId    Int
        }
    "#};

    let expect = expect![[r#"
        [1;91merror[0m: [1mError validating: Reference causes a cycle. One of the @relation attributes in this cycle must have `onDelete` and `onUpdate` referential actions set to `NoAction`. Cycle path: A.b → B.a. (Implicit default `onUpdate`: `Cascade`) Read more at https://pris.ly/d/cyclic-referential-actions[0m
          [1;94m-->[0m  [4mschema.prisma:13[0m
        [1;94m   | [0m
        [1;94m12 | [0m    id     Int  @id @default(autoincrement())
        [1;94m13 | [0m    [1;91mb      B    @relation(name: "foo", fields: [bId], references: [id], onDelete: Cascade)[0m
        [1;94m14 | [0m    bId    Int
        [1;94m   | [0m
        [1;91merror[0m: [1mError validating: Reference causes a cycle. One of the @relation attributes in this cycle must have `onDelete` and `onUpdate` referential actions set to `NoAction`. Cycle path: B.a → A.b. Read more at https://pris.ly/d/cyclic-referential-actions[0m
          [1;94m-->[0m  [4mschema.prisma:20[0m
        [1;94m   | [0m
        [1;94m19 | [0m    id     Int @id @default(autoincrement())
        [1;94m20 | [0m    [1;91ma      A   @relation(name: "bar", fields: [aId], references: [id], onUpdate: Cascade)[0m
        [1;94m21 | [0m    as     A[] @relation(name: "foo")
        [1;94m   | [0m
    "#]];

    expect.assert_eq(&parse_unwrap_err(dml));
}
