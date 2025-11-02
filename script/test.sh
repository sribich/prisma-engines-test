#!/usr/bin/env bash

function schemaEngineMatrix() {
    local -A dbs

    dbs["mysql_5_6"]="mysql://root:prisma@localhost:3309"
    dbs["mysql_5_7"]="mysql://root:prisma@localhost:3306"
    dbs["mysql_mariadb"]="mysql://root:prisma@localhost:3308"
    dbs["postgres12"]="postgresql://postgres:prisma@localhost:5434"
    dbs["postgres13"]="postgresql://postgres:prisma@localhost:5435"
    dbs["postgres14"]="postgresql://postgres:prisma@localhost:5437"
    dbs["postgres15"]="postgresql://postgres:prisma@localhost:5438"

    for database in "${!dbs[@]}"; do
        local url="${dbs[$database]}"

        $1 "$database" "$url"
    done
}

function testQuaint() {
    :
}

function testBlackBox() {
    :
}

function testQueryEngine() {
    cargo test --all-features --package=query-engine
}

function testQueryEngineTests() {
    cargo test --all-features --package=query-engine-tests
}

function testSqlMigration() {
    :
}

function testSchemaEngineCli() {
   #  cargo test --all-features --package=schema-engine-cli
   :
}

function testSqlSchemaDescriber() {
    # make "start-$1"
    TEST_DATABASE_URL="$2" cargo nextest run -p sql-schema-describer --all-features --test-threads=8

}

function testSqlIntrospection() {
    :
}


# make test-unit

testQuaint
testBlackBox
# testQueryEngine
# testQueryEngineTests
testSqlMigration
testSchemaEngineCli
schemaEngineMatrix testSqlSchemaDescriber
testSqlIntrospection