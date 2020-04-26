#!/bin/sh

# Addons

# Antibody https://getantibody.github.io/

# total reinstall, it errors out too much
rm -rf "$XDG_CACHE_HOME/antibody"

echo "Installing Antibody"
mkdir -p "$ZDOTDIR/bin"
curl -sfL git.io/antibody | sh -s - -b "$ZDOTDIR/bin"

echo "Installing plugins"
antibody bundle <"$ZDOTDIR/.zsh-plugins.antibody" \
	>"$ZDOTDIR/.zsh-plugins.antibody.sh"

# TODO: call update and move update
echo "Updating plugins"
antibody update

echo "Fixing permissions"

chmod -R 700 "$XDG_CACHE_HOME/antibody"
chmod -R 700 "$XDG_CONFIG_HOME/zsh"

# TODO: Apply zsh-defer source to applicable plugins
