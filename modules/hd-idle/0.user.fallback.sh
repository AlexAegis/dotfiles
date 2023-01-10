#!/bin/sh

HD_IDLE_DIR="$XDG_DATA_HOME/hdidle"

if ! [ -d "$HD_IDLE_DIR/.git" ]; then
	git clone https://github.com/adelolmo/hd-idle.git "$HD_IDLE_DIR"
else
	(
		cd "$HD_IDLE_DIR" || exit 1
		git pull
	)
fi

cd "$HD_IDLE_DIR" || exit 1
make

# TODO: It should install the systemd services, but on debian systems there is
# a proper package and for Arch there is an AUR package.
