#!/bin/sh

pacman -Syu --needed --noconfirm wine winetricks

# Sound won't work without these on Arch
pacman -Syu --needed --noconfirm lib32-alsa-plugins lib32-libpulse lib32-openal
