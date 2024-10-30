#!/bin/sh

rustup install nightly
rustup default stable

rustup target install wasm32-unknown-unknown

rustup component add rls
rustup component add rustfmt

# Cargo extensions
cargo install cargo-update
cargo install cargo-tree
cargo install cargo-edit
cargo install cargo-expand
cargo install cargo-make
cargo install cargo-tarpaulin
