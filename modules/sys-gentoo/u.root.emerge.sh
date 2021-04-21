#!/bin/sh

if timelock emerge_sync; then
	emerge --sync
	emaint sync -A # Sync all repositories
	echo "Locking Sync for 24 hours using the emerge_sync lock"
	timelock emerge_sync 'tomorrow'
else
	echo "The emerge_sync timelock is locked,\
 it unlocks at $(timelock -p emerge_sync)"
fi
emerge --update --changed-use --deep @world
emerge --depclean

# shellcheck disable=SC1073 disable=SC1072 disable=SC1140 disable=SC1050
