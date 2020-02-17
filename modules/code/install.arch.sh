#!/bin/sh

# VS Code (Microsoft)

# Depends on 'yay'

yay -Syu visual-studio-code-bin

mkdir -p "$HOME/.config"
stow -t ~ home-code
