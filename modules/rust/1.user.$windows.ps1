#!/usr/bin/env powershell

rustup install nightly
rustup default stable

rustup target install wasm32-unknown-unknown

rustup component add rls
rustup component add rustfmt
rustup component add rust-analyzer

# Cargo extensions
cargo install cargo-tree
