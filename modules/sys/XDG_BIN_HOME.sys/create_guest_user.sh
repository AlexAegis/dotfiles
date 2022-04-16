#!/bin/sh

# This script makes sure a guest user and a group exists

if ! id guest 2>/dev/null; then
	# user does not exist
	useradd -m -U guest -p guest
	passwd -d guest
fi

# Add user to the rest of the groups
while [ "$1" ]; do
	if ! getent group "$1" 1>/dev/null; then
		# group doesnt exist, create one
		groupadd "$1"
	fi
	usermod -aG guest "$1"
	shift
done
