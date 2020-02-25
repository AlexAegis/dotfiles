#!/bin/sh

SCRIPT_PATH=${0%/*}

# Presourcing the environment file for the installation
mkdir -p "$HOME/bin"
mkdir -p "$HOME/.config"
stow -d $SCRIPT_PATH -t $HOME home-zsh
# TODO: refactor this so zshenv is accessible as $HOME/.zshenv
set -a
. $SCRIPT_PATH/home-zsh/.zshenv
set +a

# Addons

# Antibody https://getantibody.github.io/

echo Installing Antibody
mkdir -p "$ZDOTDIR/tools"
curl -sfL git.io/antibody | sh -s - -b $ZDOTDIR/tools

echo Installing plugins
antibody bundle <$ZDOTDIR/.zsh-plugins.antibody >$ZDOTDIR/.zsh-plugins.antibody.sh

echo Updating plugins
antibody update

echo Fixing permissions
chmod 700 $HOME/.cache/antibody/*

# TODO: Apply zsh-defer source to applicable plugins
