#!/bin/sh

# TODOL build it in localshare and make it updateable
git clone https://aur.archlinux.org/rua.git
cd rua
makepkg -si
