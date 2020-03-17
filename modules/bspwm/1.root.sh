#!/bin/sh

echo "Creating desktop entry for bspwm"

echo '[Desktop Entry]
Name=bspwm
Comment=Binary space partitioning window manager
Exec=bspwm
Type=Application
' >/usr/share/xsessions/bspwm.desktop
