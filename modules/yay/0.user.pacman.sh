#!/bin/sh

# If it was left out on install
pacman -Syu --needed --noconfirm base-devel

mkdir -p "$HOME/.yay"
git clone https://aur.archlinux.org/yay.git "$HOME/.yay"
cd "$HOME/.yay" || exit 1
makepkg --noconfirm --needed -si
