#!/bin/sh

rustup install nightly
rustup default stable

rustup target install wasm32-unknown-unknown

rustup component add rls
rustup component add rustfmt

# Cargo extensions
cargo install --locked cargo-update
cargo install --locked cargo-tree
cargo install --locked cargo-edit
cargo install --locked cargo-expand
cargo install --locked cargo-make
cargo install --locked cargo-tarpaulin
cargo install --locked cargo-udeps
