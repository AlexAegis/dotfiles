#!/bin/sh

# shellcheck disable=SC1091
. "./u.user.sh"

echo "Fixing permissions"
chmod -R 700 "$ZDOTDIR"

# TODO: Apply zsh-defer source to applicable plugins
