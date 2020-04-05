#!/bin/sh

while true; do
	find "$XDG_CONFIG_HOME"/environment.d/* | \
		entr -d -p systemctl --user daemon-reload
done
