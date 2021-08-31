#!/bin/sh

# If the `~/.ssh` folder is a restored one, then not removing those
# files that are present in the module and in the .ssh folder
# would cause conflicts.
mkdir -p "$HOME/.ssh/config.d"
[ -e "$HOME/.ssh/config" ] && rm "$HOME/.ssh/config"
