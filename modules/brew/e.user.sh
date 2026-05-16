#!/bin/sh

if [ -f "$XDG_CONFIG_HOME/environment.d/40-brew.conf" ]; then
	# shellcheck disable=SC1091
	. "$XDG_CONFIG_HOME/environment.d/40-brew.conf"
fi

if [ -f "$XDG_CONFIG_HOME/rc.d/40-brew.sh" ]; then
	# shellcheck disable=SC1091
	. "$XDG_CONFIG_HOME/rc.d/40-brew.sh"
fi
