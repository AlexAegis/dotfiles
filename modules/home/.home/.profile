#!/bin/sh

# POSIX compliant Defaults sourced from every other shell
#
# Load environmental variables and aliases
# shellcheck disable=SC1090
[ -e "$HOME/bin/loadenv" ] && QUIET=1 . "$HOME/bin/loadenv"
