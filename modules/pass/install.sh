#!/bin/sh

git init --bare ~/.password-store

mkdir -p "$HOME/.config"
stow -t ~ home-pass

# TODO: # pass git remote add origin user@server:~/.password-store
