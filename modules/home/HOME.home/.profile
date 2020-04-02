#!/bin/sh

[ ! "$QUIET" ] && echo "Loading .profile"

# POSIX compliant Defaults sourced from every other shell
#
# Load environmental variables and aliases
set -a
# preload xdg env so loadenv can load the rest according to that
# shellcheck disable=SC1090
echo ".propfile xch: $XDG_CONFIG_HOME"
echo ".propfile xch: $XDG_BIN_HOME"
# . "$HOME/.config/environment.d/10-xdg.conf"
# shellcheck disable=SC1090
[ -e "$XDG_BIN_HOME/loadenv" ] && . "$XDG_BIN_HOME/loadenv"
set +a
