#!/bin/sh

rustup install nightly
rustup default stable

rustup component add rls

# Cargo extensions
cargo install cargo-tree
