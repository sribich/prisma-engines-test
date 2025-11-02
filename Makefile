CONFIG_PATH = ./query-engine/connector-test-kit-rs/test-configs
CONFIG_FILE = .test_config

default: build

###############
# clean tasks #
###############
clean-cargo:
	@echo "Cleaning cargo" && \
	cargo clean

clean: clean-cargo

###################
# script wrappers #
###################

bootstrap-darwin:
	script/bootstrap-darwin

profile-shell:
	script/profile-shell

##################
# Build commands #
##################
build:
	cargo build

build-qe:
	cargo build --package query-engine

# Emulate pedantic CI compilation.
pedantic:
	cargo fmt -- --check
	cargo clippy --all-features --all-targets -- -Dwarnings

release:
	cargo build --release

#################
# Test commands #
#################
test: test-unit test-quaint test-query-engine test-sql-introspection test-sql-schema-describer test-schema-engine-cli test-sql-migration test-black-box-tests

test-unit:
	cargo test --workspace --all-features \
	    --exclude=quaint \
	    --exclude=query-engine \
	    --exclude=black-box-tests \
	    --exclude=query-engine-tests \
	    --exclude=sql-migration-tests \
	    --exclude=schema-engine-cli \
	    --exclude=sql-schema-describer \
	    --exclude=sql-introspection-tests

test-quaint: export TEST_MYSQL = mysql://root:prisma@localhost:3306/prisma
test-quaint: export TEST_MYSQL8 = mysql://root:prisma@localhost:3307/prisma
test-quaint: export TEST_MYSQL_MARIADB = mysql://root:prisma@localhost:3308/prisma
test-quaint: export TEST_PSQL = postgresql://postgres:prisma@localhost:5435/postgres
test-quaint:
	cargo test --package quaint --all-features

test-query-engine:
	TEST_CONNECTOR=postgres TEST_CONNECTOR_VERSION=16 cargo test --package query-engine-tests --all-features

test-sql-introspection:
	TEST_DATABASE_URL="postgresql://postgres:prisma@localhost:5439" cargo test --package=sql-introspection-tests --all-features

test-sql-schema-describer:
	TEST_DATABASE_URL="postgresql://postgres:prisma@localhost:5439" cargo test --package=sql-schema-describer --all-features

test-schema-engine-cli:
	# TEST_DATABASE_URL="postgresql://postgres:prisma@localhost:5439" cargo test --package=schema-engine-cli --all-features

test-sql-migration:
	TEST_DATABASE_URL="postgresql://postgres:prisma@localhost:5439" cargo test --package=sql-migration-tests --all-features

test-black-box-tests: build-qe
	TEST_CONNECTOR=postgres TEST_CONNECTOR_VERSION=16 cargo test --package=black-box-tests --all-features -- --test-threads=1


test-qe-verbose:
	cargo test --package query-engine-tests -- --nocapture

# Single threaded thread execution.
test-qe-st:
	cargo test --package query-engine-tests -- --test-threads 1

# Single threaded thread execution, verbose.
test-qe-verbose-st:
	cargo test --package query-engine-tests -- --nocapture --test-threads 1

###########################
# Database setup commands #
###########################
all-dbs-up:
	docker compose -f docker-compose.yml up --wait -d --remove-orphans

all-dbs-down:
	docker compose -f docker-compose.yml down -v --remove-orphans

start-sqlite:

dev-sqlite:
	cp $(CONFIG_PATH)/sqlite $(CONFIG_FILE)

dev-react-native:
	cp $(CONFIG_PATH)/react-native $(CONFIG_FILE)

dev-d1:
	cp $(CONFIG_PATH)/cloudflare-d1 $(CONFIG_FILE)

test-d1: dev-d1 test-qe-st

start-postgres12:
	docker compose -f docker-compose.yml up --wait -d --remove-orphans postgres12

dev-postgres12: start-postgres12
	cp $(CONFIG_PATH)/postgres12 $(CONFIG_FILE)

start-postgres13:
	docker compose -f docker-compose.yml up --wait -d --remove-orphans postgres13

dev-postgres13: start-postgres13
	cp $(CONFIG_PATH)/postgres13 $(CONFIG_FILE)

start-postgres14:
	docker compose -f docker-compose.yml up --wait -d --remove-orphans postgres14

dev-postgres14: start-postgres14
	cp $(CONFIG_PATH)/postgres14 $(CONFIG_FILE)

start-postgres15:
	docker compose -f docker-compose.yml up --wait -d --remove-orphans postgres15

dev-postgres15: start-postgres15
	cp $(CONFIG_PATH)/postgres15 $(CONFIG_FILE)

start-postgres16:
	docker compose -f docker-compose.yml up -d --remove-orphans postgres16

dev-postgres16: start-postgres16
	cp $(CONFIG_PATH)/postgres16 $(CONFIG_FILE)

dev-pgbouncer:
	docker compose -f docker-compose.yml up --wait -d --remove-orphans pgbouncer postgres11

start-mysql_5_7:
	docker compose -f docker-compose.yml up --wait -d --remove-orphans mysql-5-7

dev-mysql: start-mysql_5_7
	cp $(CONFIG_PATH)/mysql57 $(CONFIG_FILE)

start-mysql_5_6:
	docker compose -f docker-compose.yml up --wait -d --remove-orphans mysql-5-6

dev-mysql_5_6: start-mysql_5_6
	cp $(CONFIG_PATH)/mysql56 $(CONFIG_FILE)

start-mysql_8:
	docker compose -f docker-compose.yml up --wait -d --remove-orphans mysql-8-0

dev-mysql8: start-mysql_8
	cp $(CONFIG_PATH)/mysql8 $(CONFIG_FILE)

start-mysql_mariadb:
	docker compose -f docker-compose.yml up --wait -d --remove-orphans mariadb-10-0

start-mysql_mariadb_11:
	docker compose -f docker-compose.yml up --wait -d --remove-orphans mariadb-11-0

dev-mariadb: start-mysql_mariadb
	cp $(CONFIG_PATH)/mariadb $(CONFIG_FILE)

dev-mariadb11: start-mysql_mariadb_11
	cp $(CONFIG_PATH)/mariadb $(CONFIG_FILE)

start-vitess_8_0:
	docker compose -f docker-compose.yml up --wait -d --remove-orphans vitess-test-8_0 vitess-shadow-8_0

dev-vitess_8_0: start-vitess_8_0
	cp $(CONFIG_PATH)/vitess_8_0 $(CONFIG_FILE)


######################
# Local dev commands #
######################
qe:
	cargo run --bin query-engine -- --engine-protocol json --enable-raw-queries --enable-metrics --enable-open-telemetry --enable-telemetry-in-response

qe-graphql:
	cargo run --bin query-engine -- --engine-protocol graphql --enable-playground --enable-raw-queries --enable-metrics --enable-open-telemetry --enable-telemetry-in-response

qe-dmmf:
	cargo run --bin query-engine -- cli dmmf > dmmf.json

show-metrics:
	docker compose -f docker-compose.yml up --wait -d --remove-orphans grafana prometheus

## OpenTelemetry
otel:
	docker compose up --remove-orphans -d otel
