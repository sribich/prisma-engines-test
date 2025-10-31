REPO_ROOT := $(shell git rev-parse --show-toplevel)

CONFIG_PATH = ./query-engine/connector-test-kit-rs/test-configs
CONFIG_FILE = .test_config
SCHEMA_EXAMPLES_PATH = ./query-engine/example_schemas
DEV_SCHEMA_FILE = dev_datamodel.prisma
PRISMA_BRANCH ?= main
ENGINE_SIZE_OUTPUT ?= /dev/stdout

LIBRARY_EXT := $(shell                            \
    case "$$(uname -s)" in                        \
        (Darwin)               echo "dylib" ;;    \
        (MINGW*|MSYS*|CYGWIN*) echo "dll"   ;;    \
        (*)                    echo "so"    ;;    \
    esac)

PROFILE ?= dev

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
test: export TEST_MYSQL = mysql://root:prisma@localhost:3306/prisma
test: export TEST_MYSQL8 = mysql://root:prisma@localhost:3307/prisma
test: export TEST_MYSQL_MARIADB = mysql://root:prisma@localhost:3308/prisma
test: export TEST_PSQL = postgresql://postgres:prisma@localhost:5435/postgres
test:
    # cargo test --package=quaint --all-features
	# cargo test --package=query-engine-tests --all-features
	# cargo test --package=sql-introspection-tests --all-features
	# cargo test --package=sql-schema-describer --all-features
	# cargo test --package=schema-engine-cli --all-features
	# cargo test --package=sql-migration-tests --all-features
	# cargo test --package=black-box-tests --all-features
	
	
	cargo test --workspace --all-features \
	    --exclude=quaint
	    --exclude=black-box-tests \
	    --exclude=query-engine-tests \
	    --exclude=sql-migration-tests \
	    --exclude=schema-engine-cli \
	    --exclude=sql-schema-describer \
	    --exclude=sql-introspection-tests

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

test-quaint:
	cargo test --package quaint --all-features

test-qe:
	cargo test --package query-engine-tests

test-qe-verbose:
	cargo test --package query-engine-tests -- --nocapture

# Single threaded thread execution.
test-qe-st:
	cargo test --package query-engine-tests -- --test-threads 1

# Single threaded thread execution, verbose.
test-qe-verbose-st:
	cargo test --package query-engine-tests -- --nocapture --test-threads 1

# Black-box tests, exercising the query engine HTTP apis (metrics, tracing, etc)
test-qe-black-box: build-qe
	cargo test --package black-box-tests -- --test-threads 1

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

dev-libsql-qc: build-qc-wasm build-driver-adapters-kit-qc
	cp $(CONFIG_PATH)/libsql-qc $(CONFIG_FILE)

test-libsql-qc: dev-libsql-qc test-qe-st

dev-better-sqlite3-qc: build-qc-wasm build-driver-adapters-kit-qc
	cp $(CONFIG_PATH)/better-sqlite3-qc $(CONFIG_FILE)

test-better-sqlite3-qc: dev-better-sqlite3-qc test-qe-st

dev-d1-qc: build-qc-wasm build-driver-adapters-kit-qc
	cp $(CONFIG_PATH)/d1-qc $(CONFIG_FILE)

test-d1-qc: dev-d1-qc test-qe-st

dev-d1: build-qe-wasm build-se-wasm build-driver-adapters-kit-qe
	cp $(CONFIG_PATH)/cloudflare-d1 $(CONFIG_FILE)

test-d1: dev-d1 test-qe-st
test-driver-adapter-d1: test-d1

dev-better-sqlite3-wasm: build-qe-wasm build-se-wasm build-driver-adapters-kit-qe
	cp $(CONFIG_PATH)/better-sqlite3-wasm $(CONFIG_FILE)

test-better-sqlite3-wasm: dev-better-sqlite3-wasm test-qe-st
test-driver-adapter-better-sqlite3-wasm: test-better-sqlite3-wasm

start-postgres12:
	docker compose -f docker-compose.yml up --wait -d --remove-orphans postgres12

dev-postgres12: start-postgres12
	cp $(CONFIG_PATH)/postgres12 $(CONFIG_FILE)

start-postgres13:
	docker compose -f docker-compose.yml up --wait -d --remove-orphans postgres13

dev-postgres13: start-postgres13
	cp $(CONFIG_PATH)/postgres13 $(CONFIG_FILE)

dev-pg-qc: start-postgres13 build-qc-wasm build-driver-adapters-kit-qc
	cp $(CONFIG_PATH)/pg-qc $(CONFIG_FILE)

dev-pg-qc-join:
	PRISMA_RELATION_LOAD_STRATEGY=join make dev-pg-qc

dev-pg-qc-query:
	PRISMA_RELATION_LOAD_STRATEGY=query make dev-pg-qc

test-pg-qc: dev-pg-qc test-qe

test-pg-qc-join:
	PRISMA_RELATION_LOAD_STRATEGY=join make test-pg-qc

test-pg-qc-query:
	PRISMA_RELATION_LOAD_STRATEGY=query make test-pg-qc

test-driver-adapter-pg: test-pg-js
test-driver-adapter-pg-wasm: test-pg-wasm

start-pg-bench:
	docker compose -f libs/driver-adapters/executor/bench/docker-compose.yml up --wait -d --remove-orphans postgres

setup-pg-bench: start-pg-bench build-qe-napi build-qe-wasm build-driver-adapters-kit-qe

run-bench:
	DATABASE_URL="postgresql://postgres:postgres@localhost:5432/bench?schema=imdb_bench&sslmode=disable" \
	node --experimental-wasm-modules libs/driver-adapters/executor/dist/bench.mjs

bench-pg-js: setup-pg-bench run-bench

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

dev-mariadb-mysql-qc: start-mysql_8 build-qc-wasm build-driver-adapters-kit-qc
	cp $(CONFIG_PATH)/mariadb-mysql-qc $(CONFIG_FILE)

test-mariadb-mysql-qc: dev-mariadb-mysql-qc test-qe-st

dev-mariadb-qc: start-mysql_mariadb build-qc-wasm build-driver-adapters-kit-qc
	cp $(CONFIG_PATH)/mariadb-qc $(CONFIG_FILE)

test-mariadb-qc: dev-mariadb-qc test-qe-st

######################
# Local dev commands #
######################
install-driver-adapters-kit-deps: build-driver-adapters
	cd libs/driver-adapters && pnpm i

build-driver-adapters-kit: install-driver-adapters-kit-deps
	cd libs/driver-adapters && pnpm build

build-driver-adapters-kit-qe: install-driver-adapters-kit-deps
	cd libs/driver-adapters && pnpm build:qe

build-driver-adapters-kit-qc: install-driver-adapters-kit-deps
	cd libs/driver-adapters && pnpm build:qc

build-driver-adapters: ensure-prisma-present
	@echo "Building driver adapters..."
	@cd ../prisma && pnpm i
	@echo "Driver adapters build completed.";

ensure-prisma-present:
	@if [ -d ../prisma ]; then \
		cd "$(realpath ../prisma)" && git fetch origin main; \
		LOCAL_CHANGES=$$(git diff --name-only HEAD origin/main -- 'packages/*adapter*'); \
		if [ -n "$$LOCAL_CHANGES" ]; then \
		  echo "⚠️ ../prisma diverges from prisma/prisma main branch. Test results might diverge from those in CI ⚠️ "; \
		fi \
	else \
		echo "git clone --depth=1 https://github.com/prisma/prisma.git --branch=$(PRISMA_BRANCH) ../prisma"; \
		git clone --depth=1 https://github.com/prisma/prisma.git --branch=$(PRISMA_BRANCH) "../prisma" && echo "Prisma repository has been cloned to ../prisma"; \
	fi;

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

# Build the debug version of Query Engine Node-API library ready to be consumed by Node.js
.PHONY: qe-node-api
qe-node-api: build target/debug/libquery_engine.node --profile=$(PROFILE)

%.node: %.$(LIBRARY_EXT)
# Remove the file first to work around a macOS bug: https://openradar.appspot.com/FB8914243
# otherwise macOS gatekeeper may kill the Node.js process when it tries to load the library
	if [[ "$$(uname -sm)" == "Darwin arm64" ]]; then rm -f $@; fi
	cp $< $@
