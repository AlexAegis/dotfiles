#!/bin/sh

if ! [ "$(id -u)" = 0 ]; then
   echo "zsh can't be installed without root!"
   # TODO: only exit 1 when a dot variable is not set if dot run with sudo maybe
   # TODO:
   exit 1
fi

# Install package using pacman
pacman -Syu --needed --noconfirm zsh
