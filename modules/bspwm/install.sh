#!/bin/sh

# Tiling Window Manager
# Arch example https://wiki.archlinux.org/index.php/Bspwm

pacman -Syu --needed --noconfirm bspwm

stow -t ~ home-bspwm
