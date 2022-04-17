#!/bin/sh

# Timelock better not run as root

if timelock emerge_sync; then
	sudo emerge --sync
	sudo emaint sync -A # Sync all repositories
	echo "Locking Sync for 24 hours using the emerge_sync lock"
	timelock emerge_sync 'tomorrow'
else
	echo "The emerge_sync timelock is locked,\
 it unlocks at $(timelock -p emerge_sync)"
fi
sudo emerge --update --changed-use --deep @world
sudo emerge --depclean
