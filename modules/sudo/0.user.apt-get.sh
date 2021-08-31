#!/bin/sh

command='apt-get install -y sudo'

if command -v sudo >/dev/null; then
	sudo sh -c "$command"
else
	su -c "$command"
fi
