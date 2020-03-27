#!/bin/sh

pacman -Syu --needed --noconfirm \
	xorg-server \
	xorg-xrandr \
	arandr \
	xf86-input-evdev \
	usbutils
