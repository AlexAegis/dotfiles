#!/bin/sh

while true; do
	find "$XDG_CONFIG_HOME"/entr/watchers/* | \
		entr -d -p systemctl --user restart entr-watch.service
done
