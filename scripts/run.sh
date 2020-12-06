#!/bin/bash
PACKAGE_NAME=$(head Cargo.toml | awk '/^name/{print $3}' | tr -d '"' | tr -d "'")
cd ./dist
RANDOM_TOKEN=$(dd if=/dev/urandom bs=1 count=30 2>/dev/null | base64 | sed 's/[^a-zA-Z0-9]//g')

BP_SERVER_TOKEN="${RANDOM_TOKEN}" ./${PACKAGE_NAME}
