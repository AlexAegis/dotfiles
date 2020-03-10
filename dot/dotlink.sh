#!/bin/sh

DOT_PATH=$(
	cd "${0%/*}" || exit
	pwd
)

# Using a symlink to make dot available without modifying the PATH
ln -sf "$DOT_PATH/dot/dot.sh" "/usr/local/bin/dot"
