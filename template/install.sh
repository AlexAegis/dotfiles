#!/bin/sh
# Module scruture is fixed, this way the script can be called from anywhere
export SCRIPT_PATH=${0%/*}
# ? Can even cd into it
cd "${0%/*}" || exit

# This script should never require root priviliges

# ? When stowing always specify the directory as the scripts
# ? because they are running in a subshell
stow -t "$HOME" -d "${0%/*}" .template
