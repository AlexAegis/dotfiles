#!/bin/sh

DOT_PATH=$(
	cd "${0%/*}" || exit
	pwd
)

# Using a symlink to make dot available without modifying the PATH
# But only when using sudo

if [ "$SUDO_USER" ]; then
	ln -sf "$DOT_PATH/src/dot.sh" "/usr/local/bin/dot"
fi

# If sourced, will expand the path with itself
export PATH="$PATH:$DOT_PATH/"
