#!/bin/sh

# Conflicts in the xorg bundle, --noconfirm is removed
pacman -Syu --needed \
	xorg \
	xorg-xinit \
	autorandr \
	arandr \
	xf86-input-evdev \
	usbutils \
	xterm
