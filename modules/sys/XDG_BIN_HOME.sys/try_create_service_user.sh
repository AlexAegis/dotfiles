#!/bin/sh

# This script makes sure a user and a group exists

if [ -z "$1" ]; then
	echo "Pass a username as the first argument"
	exit 1
fi

username="$1"

shift

if ! id "$username" 2>/dev/null; then
	# user does not exist
	if ! getent group "$username" 1>/dev/null; then
		# group doesnt exist, create one with the user
		if [ -z "${CREATE_WITH_HOME}" ]; then
			# With a home folder
			useradd -m -U "$username"
		else
			# Without
			useradd -M -U "$username"
		fi
	else
		# group exists, do not create one for the user, just add them to it
		if [ -z "${CREATE_WITH_HOME}" ]; then
			# With a home folder
			useradd -m -N "$username"
		else
			# Without
			useradd -M -N "$username"
		fi
		usermod -aG "$username" "$username"
	fi
else
	# user already exists, make sure it's linked to the group
	if ! getent group "$username" 1>/dev/null; then
		# group doesnt exist, create one
		groupadd "$username"
	fi
	usermod -aG "$username" "$username"
fi

# Add user to the rest of the groups
while [ "$1" ]; do
	if ! getent group "$1" 1>/dev/null; then
		# group doesnt exist, create one
		groupadd "$1"
	fi
	usermod -aG "$username" "$1"
	shift
done
