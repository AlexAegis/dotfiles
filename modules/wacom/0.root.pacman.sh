#!/bin/sh

# uses on Xorg
if command -v Xorg >/dev/null; then
	pacman -Syu --needed --noconfirm xf86-input-wacom

	if command -v plasma_session >/dev/null; then
		pacman -Syu --needed --noconfirm kcm-wacomtablet
	fi
fi
