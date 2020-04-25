#!/bin/sh
PACKAGE_NAME=$(head Cargo.toml | awk '/^name/{print $3}' | tr -d '"' | tr -d "'")
cargo build --release --target $(rustup target list | awk '/musl.*installed/{print $1}')
mkdir -p ./dist
cp ./target/x86_64-unknown-linux-musl/release/${PACKAGE_NAME} ./dist/${PACKAGE_NAME}
strip ./dist/${PACKAGE_NAME}
