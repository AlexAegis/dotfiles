#!/bin/sh

# Haven't got it to work on a system level. When using a user level barrier
# client I have to rely on the display manager's autologin feature to log in.
# WARNING: secondary screen unavailable: unable to open screen

# cp ./resources/barrier.service /etc/systemd/system/barrier.service
#
# systemctl daemon-reload
# systemctl enable --now barrier
#
# useradd -M -U barrier
