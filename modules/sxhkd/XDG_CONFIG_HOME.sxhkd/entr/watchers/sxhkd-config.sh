#!/bin/sh

while true; do
	find "$XDG_CONFIG_HOME"/sxhkd/sxhkdrc.d/* \
	     "$XDG_CONFIG_HOME"/sxhkd/sxhkdrc | \
		entr -d -p systemctl --user restart sxhkd.service
done
