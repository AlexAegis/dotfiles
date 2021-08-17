#!/bin/sh

set -a
#shellcheck disable=SC1091
LOAD_CONF=1 . "$HOME/.profile"
export PLASMA_ENV_LOADED=1
set +a

#if command -v dbus-update-activation-environment >/dev/null 2>&1; then
#    dbus-update-activation-environment --all --systemd
#fi
