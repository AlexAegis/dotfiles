#!/bin/sh

command='pacman -Syu --needed --noconfirm sudo'

if command -v sudo >/dev/null; then
	sudo sh -c "$command"
else
	su -c "$command"
fi
