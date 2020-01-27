#!/bin/bash

# Update packages

if [ $(which apt-get) ]; then
	apt-get -yq update
	apt-get -yq upgrade
	apt-get -yq autoremove
else
	echo No package manager found
fi
