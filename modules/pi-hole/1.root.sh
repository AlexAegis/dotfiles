#!/bin/sh

# shellcheck disable=SC1091
. "./u.root.sh"

pihole -a -p # Ask for a password for the admin panel
pihole -a -c # Use Celsius
pihole -a -e $(git config user.email) # Set contact mail
