#!/usr/bin/env bash

function test() {
    cd "$1"

    if [[ "$2" == "true" ]]; then
        docker compose up -d
        sleep 5
        cargo test --all-features
        docker compose down
    else
        cargo test --all-features
    fi

    cd -
}

test "prisma-fmt"
test "psl/psl"
test "quaint" "true"
