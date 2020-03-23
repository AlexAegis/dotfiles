#!/bin/sh

# pacman is not necessarily needed but its useless without it
mkdir -p "$HOME/.yay"
git clone https://aur.archlinux.org/yay.git "$HOME/.yay"
cd "$HOME/.yay" || exit 1
makepkg --noconfirm --needed -si
