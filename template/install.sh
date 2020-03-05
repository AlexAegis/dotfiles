#!/bin/sh
# Module path can be accessed through $PWD because dot changes directory to
# the module before running anything
echo "$MODULE_PATH"
# ? Can even cd into it
cd "$MODULE_PATH" || exit

# This script should never require root priviliges

# ? When stowing always specify the directory as the scripts
# ? because they are running in a subshell
# ? stowing is handled by dot so you don't have to include this in your script
# stow -t "$HOME" -d "${0%/*}" .template
