#!/bin/sh

# shellcheck disable=SC1091
. "./XDG_CONFIG_HOME.xdg-user-dirs/environment.d/11-xdg-user-dirs.conf"

if [ "$mac" ]; then
	# shellcheck disable=SC1091
	. "./XDG_CONFIG_HOME.\$mac.xdg-user-dirs/environment.d/12-xdg-user-dirs-mac.conf"
fi
