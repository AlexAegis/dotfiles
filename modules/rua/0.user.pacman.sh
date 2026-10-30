#!/bin/sh

# TODO build it in localshare and make it updateable
[ -d rua/.git ] || git clone https://aur.archlinux.org/rua.git || exit 1
cd rua || exit 1
makepkg -si
