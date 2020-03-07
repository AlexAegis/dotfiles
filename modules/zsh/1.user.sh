#!/bin/sh

# Presourcing the environment file for the installation
# dot links every file to $HOME before this script
set -a
# shellcheck disable=SC1090
. "$HOME/.zshenv"
set +a

# Addons

# Antibody https://getantibody.github.io/

echo "Installing Antibody"
mkdir -p "$ZDOTDIR/tools"
curl -sfL git.io/antibody | sh -s - -b "$ZDOTDIR/tools"

echo "Installing plugins"
antibody bundle <"$ZDOTDIR/.zsh-plugins.antibody" >"$ZDOTDIR/.zsh-plugins.antibody.sh"

# TODO: call update and move update
echo "Updating plugins"
antibody update

echo "Fixing permissions"
chmod 700 "$HOME"/.cache/antibody/*

# TODO: Apply zsh-defer source to applicable plugins
