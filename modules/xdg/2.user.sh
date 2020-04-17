#!/bin/sh

# See 1.root.sh for the reason of this check
if [ -e /etc/user-dirs.conf ] && [ -e /etc/user-dirs.defaults ]; then
	xdg-user-dirs-update
fi
