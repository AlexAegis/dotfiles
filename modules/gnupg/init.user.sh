#!/bin/sh

# Because of the backups
mkdir -p "$XDG_CONFIG_HOME/gnupg"
chmod 700 "$XDG_CONFIG_HOME/gnupg"

if [ -d "$HOME/.gnupg" ]; then
	if [ -d "$XDG_CACHE_HOME/gnupgbak" ]; then
		rm -r "$XDG_CACHE_HOME/gnupgbak"
	fi

	mkdir -p "$XDG_CACHE_HOME/gnupgbak"
	mv "$HOME/.gnupg" "$XDG_CACHE_HOME/gnupgbak"
fi
