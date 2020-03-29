#!/bin/sh

pacman -Syu --needed --noconfirm pulseaudio pulseaudio-alsa \
	pulseaudio-bluetooth pulseaudio-equalizer alsa-utils
