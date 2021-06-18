#!/bin/sh

default_config_home="$HOME/.config/neofetch"

# if it's a directory and not a symlink
if [ -d "$default_config_home" ]; then
	rm -r "$default_config_home"
fi
