#!/bin/sh

# Presourcing so it installs to the right location
# shellcheck disable=SC1091
set -a
. "./XDG_CONFIG_HOME.rust/environment.d/20-rust.conf"
set +a
export PATH="$CARGO_HOME/bin:$PATH"
echo "rust env $PATH"

