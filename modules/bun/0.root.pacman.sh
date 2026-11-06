#!/bin/sh

# ? Downloading first lets the install after the removal of bun-bin run offline
pacman -Syuw --needed --noconfirm bun || exit 1
if pacman -Q bun-bin >/dev/null 2>&1; then
	# ? --noconfirm declines the removal of bun-bin, which conflicts with bun
	pacman -Rdd --noconfirm bun-bin || exit 1
fi
pacman -Su --needed --noconfirm bun
