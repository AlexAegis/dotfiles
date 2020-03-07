#!/bin/sh

if [ "$(id -u)" != 0 ]; then
	echo "zsh can't be installed without root!"
	exit 1
fi

# Install package using pacman
pacman -Syu --needed --noconfirm zsh
