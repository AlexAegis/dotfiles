#!/bin/sh

echo "Starting entr watchers" | systemd-cat
for script in "$XDG_CONFIG_HOME"/entr/watchers/*.sh; do
	echo "Starting entr watcher: $script" | systemd-cat
	"$script" &
done
