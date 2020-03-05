#!/bin/sh

# ? Not used here
# stow -t "$HOME" -d "${0%/*}" .base

mkdir -p \
	"$HOME/.env.d" \
	"$HOME/.ec.d" \
	"$HOME/.alias.d"
