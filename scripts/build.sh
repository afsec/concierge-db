#!/bin/bash

mkdir -p ./dist
PACKAGE_NAME=$(awk '/^name/{print $3}' Cargo.toml | tr -d '"')

if [ X"$1" == X"-d" ]; then
    rm -f ./target/x86_64-unknown-linux-musl/debug/${PACKAGE_NAME}
    cargo build --package ${PACKAGE_NAME} --target $(rustup target list | awk '/musl.*installed/{print $1}') &&
    strip ./target/x86_64-unknown-linux-musl/debug/${PACKAGE_NAME} &&
    cp -v ./target/x86_64-unknown-linux-musl/debug/${PACKAGE_NAME} ./dist/${PACKAGE_NAME}

else
    rm -f ./target/x86_64-unknown-linux-musl/release/${PACKAGE_NAME}
    cargo build --release --package ${PACKAGE_NAME} --target $(rustup target list | awk '/musl.*installed/{print $1}') &&
    strip ./target/x86_64-unknown-linux-musl/release/${PACKAGE_NAME} &&
    cp -v ./target/x86_64-unknown-linux-musl/release/${PACKAGE_NAME} ./dist/${PACKAGE_NAME}

fi
