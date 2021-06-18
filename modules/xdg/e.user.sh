#!/bin/sh

# shellcheck disable=SC1091
. "./XDG_CONFIG_HOME.xdg/environment.d/10-xdg.conf"

if [ "$mac" ]; then
	# shellcheck disable=SC1091
	. "./XDG_CONFIG_HOME.\$mac.xdg/environment.d/11-xdg-mac.conf"
fi
