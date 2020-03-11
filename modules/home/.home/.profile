#!/bin/sh

[ ! "$QUIET" ] && echo "Loading .profile"

# POSIX compliant Defaults sourced from every other shell
#
# Load environmental variables and aliases
# set -a
# shellcheck disable=SC1090
[ -e "$HOME/bin/loadenv" ] && . "$HOME/bin/loadenv"
# set +a

export PATH="$HOME/.cargo/bin:$PATH"
