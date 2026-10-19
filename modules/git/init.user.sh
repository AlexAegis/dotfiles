#!/bin/sh

mkdir -p "$XDG_CONFIG_HOME/git/gitconfig.d"

# git config --global writes ~/.gitconfig if it exists, else the linked config
if ! [ -w ./XDG_CONFIG_HOME.git/git ]; then
	touch "$HOME/.gitconfig"
elif [ -e "$HOME/.gitconfig" ]; then
	rm "$HOME/.gitconfig"
fi
