#!/bin/sh

# Update and upgrade

# TODO Make this platform independent
# TODO Maybe run it as a cron job?

if [ $(command -v apt-get) ]; then
	apt-get -yq update
	apt-get -yq upgrade
	apt-get -yq autoremove
fi

if [ $(command -v pacman) ]; then
	pacman -Syu
fi

if [ $(command -v yay) ]; then
	yay -Syu
fi
