#!/bin/sh

pacman -Syu --needed --noconfirm \
	xorg \
	arandr \
	xf86-input-evdev \
	usbutils
