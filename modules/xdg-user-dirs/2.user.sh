#!/bin/sh

# See 1.root.sh for the reason of this check
if [ -e /etc/user-dirs.conf ] && [ -e /etc/user-dirs.defaults ]; then
	# shellcheck disable=SC1091
	. "./u.user.sh"
fi
