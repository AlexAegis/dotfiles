#!/bin/zsh

# This is here for compatibility reasons, using a login manager, this is not
# needed, but when logging directly into zsh, systemd envs are not available

[ "$VERBOSE" = 1 ] && echo "Loading .zshenv"

# ZSH Environment
. "$HOME/.config/environment.d/10-xdg.conf"
. "$XDG_CONFIG_HOME/environment.d/20-zshenv.conf"
# rest is loaded by the startup scripts in ZDOTDIR, set in 20-zshenv.conf
