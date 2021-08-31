#!/bin/sh

# These install scripts differ from the rest as they all require sudo
command='emerge -u app-admin/sudo'

if command -v sudo >/dev/null; then
	sudo sh -c "$command"
else
	su -c "$command"
fi
