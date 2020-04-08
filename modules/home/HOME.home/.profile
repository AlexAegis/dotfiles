#!/bin/sh

[ ! "$QUIET" ] && echo "Loading .profile"
# POSIX compliant Defaults sourced from every other shell
#
# Load environmental variables and aliases
set -a
# ? Even though systemd loads *.conf files and 10-xdg.conf is one of them
# ? sshd disables environment import for security reasons and it has to be
# ? loaded manually when logging in through ssh or a tty

# Respect global defaults, if any
XDG_CONFIG_HOME=${XDG_CONFIG_HOME:-$HOME/.config}

# preload xdg env so loadenv can load the rest according to that
if [ ! "$CONF_LOADED" ]; then
	# shellcheck disable=SC1090
	. "$XDG_CONFIG_HOME/environment.d/10-xdg.conf"
fi

# ? loadenv loads shell specific stuff (alias, rc drop ins) and conf files
# ? when needed
# shellcheck disable=SC1090
[ -e "$XDG_BIN_HOME/loadenv" ] && . "$XDG_BIN_HOME/loadenv"
set +a
