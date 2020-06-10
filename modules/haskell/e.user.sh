#!/bin/sh

# Only used in the install script
export GHCUP_INSTALL_BASE_PREFIX="${XDG_DATA_HOME}"
export BOOTSTRAP_HASKELL_NONINTERACTIVE="1"

# shellcheck disable=SC1091
. './XDG_CONFIG_HOME.haskell/environment.d/60-haskell.conf'
