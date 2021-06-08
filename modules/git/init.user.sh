#!/bin/sh

mkdir -p "$XDG_CONFIG_HOME/git/gitconfig.d"

if [ -e "$HOME/.gitconfig" ]; then
	rm "$HOME/.gitconfig"
fi
