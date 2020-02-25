#!/bin/sh

# VS Code (Microsoft)

# Depends on 'yay'

${SUDO_USER:+sudo -u $SUDO_USER} yay -Syu visual-studio-code-bin

mkdir -p "$HOME/.config"
stow -t ~ home-code
