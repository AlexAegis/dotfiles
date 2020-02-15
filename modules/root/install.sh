#!/bin/sh

# Enable time sync

if [ $(command -v systemctl) ]; then
	echo 'systemd detected'
	echo 'starting and enabling timesyncd service'
	systemctl enable systemd-timesyncd.service
	systemctl start systemd-timesyncd.service
fi

# Making hard folders
mkdir -p "$HOME/bin"
mkdir -p "$HOME/.config"
mkdir -p "$HOME/.env" # Don't let stow manage shared folders
mkdir -p "$HOME/.rc" # Don't let stow manage shared folders
stow -Rt ~ home
