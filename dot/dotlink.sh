#!/bin/sh

script_location=$(
	cd "${0%/*}" || exit
	pwd
)

# Using a symlink to make dot available without modifying the PATH
ln -sf "$script_location/dot.sh" "/usr/local/bin/dot"
