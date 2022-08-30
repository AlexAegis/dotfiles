#!/bin/sh

mkdir -p /etc/X11/xorg.conf.d/

# Enable inverted scrolling
# cp ./resources/90-libinput-wacom-invert-scroll.conf /etc/X11/xorg.conf.d/90-libinput-wacom-invert-scroll.conf

# TODO: Configure the tablet using xsetwacom, run the config using a systemd service
