use connection_string::JdbcString;
use quaint::{prelude::Queryable, single::Quaint};
use sql_migration_tests::multi_engine_test_api::*;
use test_macros::test_connector;

#[test_connector(tags(Postgres))]
fn soft_resets_work_on_postgres(mut api: TestApi) {
    let migrations_directory = api.create_migrations_directory();
    let mut url: url::Url = api.connection_string().parse().unwrap();

    let dm = r#"
    model Cat {
        id Int @id
        litterConsumption Int
        hungry Boolean @default(true)
    }
    "#;

    // Create the database, a first migration and the test user.
    {
        api.new_engine()
            .create_migration("01init", dm, &migrations_directory)
            .send_sync();

        let create_user = r#"
            DROP USER IF EXISTS softresetstestuser;
            CREATE USER softresetstestuser PASSWORD '1234batman' LOGIN;
            GRANT USAGE, CREATE ON SCHEMA "public" TO softresetstestuser;
        "#;

        api.raw_cmd(create_user);
    }

    let test_user_connection_string = {
        url.set_username("softresetstestuser").unwrap();
        url.set_password(Some("1234batman")).unwrap();
        url.to_string()
    };

    // Check that the test user can't drop databases.
    {
        let test_user_connection = tok(Quaint::new(&test_user_connection_string)).unwrap();
        let err = tok(test_user_connection.raw_cmd(&format!(r#"DROP DATABASE {}"#, api.test_fn_name()))).unwrap_err();

        assert_eq!(err.original_code().unwrap(), "42501"); // insufficient_privilege (https://www.postgresql.org/docs/current/errcodes-appendix.html)
    }

    // Check that the soft reset works with migrations, then with schema push.
    {
        let mut engine = api.new_engine_with_connection_strings(test_user_connection_string, None);

        engine
            .apply_migrations(&migrations_directory)
            .send_sync()
            .assert_applied_migrations(&["01init"]);

        let add_view = format!(
            r#"CREATE VIEW "{0}"."catcat" AS SELECT * FROM "{0}"."Cat" LIMIT 2"#,
            engine.schema_name(),
        );

        engine.raw_cmd(&add_view);

        engine
            .assert_schema()
            .assert_tables_count(2)
            .assert_has_table("_prisma_migrations")
            .assert_has_table("Cat");

        engine.reset().send_sync(None);
        engine.assert_schema().assert_tables_count(0);

        engine.schema_push(dm).send().assert_has_executed_steps().assert_green();

        engine.assert_schema().assert_tables_count(1).assert_has_table("Cat");

        engine.reset().send_sync(None);
        engine.assert_schema().assert_tables_count(0);
    }
}

// MySQL 5.6 doesn't have `DROP USER IF EXISTS`...
// Neither does Vitess
#[test_connector(tags(Mysql), exclude(Mysql56, Vitess))]
fn soft_resets_work_on_mysql(api: TestApi) {
    let migrations_directory = api.create_migrations_directory();
    let mut url: url::Url = api.connection_string().parse().unwrap();

    let dm = r#"
        model Cat {
            id Int @id
            litterConsumption Int
            hungry Boolean @default(true)
        }
    "#;

    {
        let mut engine = api.new_engine();

        engine.create_migration("01init", dm, &migrations_directory).send_sync();

        engine
            .apply_migrations(&migrations_directory)
            .send_sync()
            .assert_applied_migrations(&["01init"]);

        engine
            .assert_schema()
            .assert_tables_count(2)
            .assert_has_table("_prisma_migrations")
            .assert_has_table("Cat");
    }

    {
        let create_user = format!(
            r#"
            DROP USER IF EXISTS 'softresetstestuser'@'%';
            CREATE USER 'softresetstestuser'@'%' IDENTIFIED BY '1234batman';
            GRANT SELECT, USAGE, CREATE ON TABLE `{0}`.* TO 'softresetstestuser'@'%';
            GRANT DROP ON TABLE `{0}`.`Cat` TO 'softresetstestuser'@'%';
            GRANT DROP ON TABLE `{0}`.`_prisma_migrations` TO 'softresetstestuser'@'%';
            FLUSH PRIVILEGES;
        "#,
            api.test_fn_name()
        );

        api.raw_cmd(&create_user);
    }

    let test_user_connection_string = {
        url.set_username("softresetstestuser").unwrap();
        url.set_password(Some("1234batman")).unwrap();
        url.to_string()
    };

    // Check that the test user can't drop databases.
    {
        let test_user_connection = tok(Quaint::new(&test_user_connection_string)).unwrap();
        let err = tok(test_user_connection.raw_cmd(&format!(r#"DROP DATABASE `{}`"#, api.test_fn_name()))).unwrap_err();

        // insufficient_privilege
        // https://docs.oracle.com/cd/E19078-01/mysql/mysql-refman-5.1/error-handling.html
        assert_eq!(err.original_code().unwrap(), "1044");
    }

    // Check that the soft reset works with migrations, then with schema push.
    {
        let mut engine = api.new_engine_with_connection_strings(test_user_connection_string, None);

        engine.reset().send_sync(None);
        engine.assert_schema().assert_tables_count(0);

        engine.schema_push(dm).send().assert_has_executed_steps().assert_green();

        engine.assert_schema().assert_tables_count(1).assert_has_table("Cat");

        engine.reset().send_sync(None);
        engine.assert_schema().assert_tables_count(0);
    }
}
