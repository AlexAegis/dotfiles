#!/bin/sh

# shellcheck disable=SC1091
. "./XDG_CONFIG_HOME.xdg/environment.d/10-xdg.conf"

# Checking by itself, instead of utilizing the $mac variable set by pont
# so that the xdg/e.user.sh is usable on it's own
if [ "$(uname -s)" = "Darwin" ]; then
	# shellcheck disable=SC1091
	. "./XDG_CONFIG_HOME.\$mac.xdg/environment.d/11-xdg-mac.conf"
fi
