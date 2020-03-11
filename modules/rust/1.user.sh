#!/bin/sh

# loading environment before proceeding
# shellcheck disable=1091
. loadenv

rustup install nightly
rustup install stable
rustup default stable

# TODO: When 1.41 becomes stable remove install-update and use just install
# And remove -f from here
cargo install -f cargo-update
