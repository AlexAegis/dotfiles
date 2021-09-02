#!/bin/sh

# shellcheck disable=SC1091
. "./u.root.sh"

# Ask for a password for the admin panel
pihole -a -p
