#!/bin/sh

# Set up a samba password for the user if not already

existing_user=$(pdbedit -L -v | awk "/Unix username:.*${SUDO_USER:-$USER}/ { print \$3 }")

if [ -z "$existing_user" ]; then
	smbpasswd -a "${SUDO_USER:-$USER}"
else
	echo "Samba user $existing_user already exists!
To change password use smbpasswd -a <user>"
fi
