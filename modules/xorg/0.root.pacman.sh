#!/bin/sh

pacman -Syu --needed --noconfirm \
	xorg \
	xorg-xinit \
	arandr \
	xf86-input-evdev \
	usbutils
