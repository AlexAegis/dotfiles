#!/usr/bin/env powershell

rustup install nightly
rustup default stable

rustup component add rls
rustup component add rustfmt

# Cargo extensions
cargo install cargo-tree
