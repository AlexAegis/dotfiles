#!/bin/sh

mkdir -p "$XDG_CONFIG_HOME/plasma-workspace/env"
mkdir -p "$XDG_CONFIG_HOME/git/gitconfig.d"
mkdir -p "$XDG_CONFIG_HOME/menus"

# Plasma 6 on Arch provides plasma-applications.menu, while some KDE tooling
# may look for applications.menu in the user menu directory.
if [ ! -f "$XDG_CONFIG_HOME/menus/applications.menu" ]; then
	if [ -f /etc/xdg/menus/applications.menu ]; then
		cp /etc/xdg/menus/applications.menu "$XDG_CONFIG_HOME/menus/applications.menu"
	elif [ -f /etc/xdg/menus/plasma-applications.menu ]; then
		cp /etc/xdg/menus/plasma-applications.menu "$XDG_CONFIG_HOME/menus/applications.menu"
	fi
fi

# Rebuild app desktop and KDE service caches so launcher/task manager state is
# refreshed immediately after module init.
if command -v update-desktop-database >/dev/null 2>&1; then
	update-desktop-database "${XDG_DATA_HOME:-$HOME/.local/share}/applications" -v || true
fi

if command -v kbuildsycoca6 >/dev/null 2>&1; then
	kbuildsycoca6 --noincremental || true
fi
