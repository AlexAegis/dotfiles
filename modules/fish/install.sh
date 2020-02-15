#!/bin/sh
# Addons

# oh-my-fish
# https://github.com/oh-my-fish/oh-my-fish

echo Installing oh-my-fish.

curl -L https://get.oh-my.fish | fish

fish -c "omf install bobthefish" # Install powerline theme https://github.com/oh-my-fish/theme-bobthefish
fish -c "omf install foreign-env"

mkdir -p "$HOME/bin"
mkdir -p "$HOME/.config"
stow -t ~ home-fish
