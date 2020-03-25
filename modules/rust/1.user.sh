#!/bin/sh

# loading environment before proceeding
# shellcheck disable=1091
. loadenv

rustup install nightly
rustup install stable
rustup default stable

rustup component add rls
