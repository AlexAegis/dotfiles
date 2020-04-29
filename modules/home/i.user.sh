#!/bin/sh

# Also made by xdg
mkdir -p "$XDG_CONFIG_HOME/alias.d" \
		"$XDG_CONFIG_HOME/environment.d" \
		"$XDG_CONFIG_HOME/rc.d"

# This file exists by default, but do not touch it if it's already a symlink
[ -e "${HOME}/.profile" ] && [ ! -L "${HOME}/.profile" ] \
	&& mv "${HOME}/.profile" "${HOME}/.profile.bak"
