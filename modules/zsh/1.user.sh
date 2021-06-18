#!/bin/sh

# Addons

# Antibody https://getantibody.github.io/

# total reinstall, it errors out too much
rm -rf "$XDG_CACHE_HOME/antibody"

echo "Installing Antibody"
mkdir -p "$ZDOTDIR/bin"
curl -sfL git.io/antibody | sh -s - -b "$ZDOTDIR/bin"

echo "Installing plugins"
"$ZDOTDIR/bin/antibody" bundle <"$ZDOTDIR/.zsh-plugins.antibody" \
	>"$ZDOTDIR/.zsh-plugins.antibody.sh"

# shellcheck disable=SC1091
. "./u.user.sh"

echo "Fixing permissions"
chmod -R 700 "$XDG_CACHE_HOME/antibody"
chmod -R 700 "$XDG_CONFIG_HOME/zsh"
compaudit | xargs chmod g-w,o-w

# TODO: Apply zsh-defer source to applicable plugins
