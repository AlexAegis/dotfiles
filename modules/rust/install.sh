#!/bin/sh

rustup default stable

mkdir -p "$HOME/.env" # Don't let stow manage shared folders
stow -Rt ~ home-rust
