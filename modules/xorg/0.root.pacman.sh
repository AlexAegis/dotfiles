#!/bin/sh

pacman -Syu --needed --noconfirm \
	xorg \
	xorg-xinit \
	autorandr \
	arandr \
	xf86-input-evdev \
	usbutils
