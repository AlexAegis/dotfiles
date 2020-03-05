#!/bin/sh

mkdir -p "$HOME/.alias" # Don't let stow manage shared folders

stow -t "$HOME" -d "${0%/*}" .exa
