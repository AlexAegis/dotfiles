#!/bin/sh

# VS Code (Microsoft)

# Depends on 'yay'

${SUDO_USER:+sudo -u $SUDO_USER} \
	yay -Syu --needed --noconfirm visual-studio-code-bin

mkdir -p "$HOME/.config"
stow -t ~ home-code
