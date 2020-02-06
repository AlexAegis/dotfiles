#!/bin/bash

# Update and upgrade

# TODO Make this platform independent
# Maybe run it as a cron job?

if [ $(which apt-get) ]; then
	apt-get -yq update
	apt-get -yq upgrade
	apt-get -yq autoremove
fi

if [ $(which pacman) ]; then
	pacman -Syu
fi

if [ $(which yay) ]; then
	yay -Syu
fi
