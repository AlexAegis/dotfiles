#!/bin/sh

# Creating the main folders
mkdir -p "$XDG_CONFIG_HOME" \
		"$XDG_CACHE_HOME" \
		"$XDG_DATA_HOME" \
		"$XDG_BIN_HOME"

# Other common folders
mkdir -p "$XDG_CONFIG_HOME/alias.d" \
		"$XDG_CONFIG_HOME/environment.d" \
		"$XDG_CONFIG_HOME/rc.d"
