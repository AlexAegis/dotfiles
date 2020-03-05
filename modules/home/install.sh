#!/bin/sh

# Enable time sync

if [ "$(command -v systemctl)" ]; then
	echo 'systemd detected'
	echo 'starting and enabling timesyncd service'
	systemctl enable --now systemd-timesyncd.service
fi

# Making hard folders
mkdir -p "$HOME/bin"
mkdir -p "$HOME/.config"
mkdir -p "$HOME/.env.d"   # Don't let stow manage shared folders
mkdir -p "$HOME/.alias.d" # Don't let stow manage shared folders
mkdir -p "$HOME/.rc.d"    # Don't let stow manage shared folders

stow -t "$HOME" -d "${0%/*}" .home
