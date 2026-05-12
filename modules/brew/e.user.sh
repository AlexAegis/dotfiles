#!/bin/sh

if [ -f "$XDG_CONFIG_HOME/rc.d/40-brew.sh" ]; then
	# shellcheck disable=SC1091
	. "$XDG_CONFIG_HOME/rc.d/40-brew.sh"
fi
