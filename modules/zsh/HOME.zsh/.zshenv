#!/bin/zsh

# alternative solution might not needed at all, this could be sourced from
# anywhere. Left here for reference, can be deleted later.

# This is here for compatibility reasons, using a login manager, this is not
# needed, but when logging directly into zsh, systemd envs are not available

[ "$VERBOSE" = 1 ] && echo "Loading .zshenv"

if [ "$(command -v systemctl 2>/dev/null)" ]; then
	eval $(systemctl --user show-environment)
else
	# ZSH Environment
	. "$HOME/.config/environment.d/10-xdg.conf"
	. "$XDG_CONFIG_HOME/environment.d/20-zshenv.conf"
	# rest is loaded by the startup scripts in ZDOTDIR, set in 20-zshenv.conf
fi
