#!/bin/sh

# TODO build it in localshare and make it updateable
git clone https://aur.archlinux.org/rua.git
cd rua || exit 1
makepkg -si
