#!/bin/sh

# Enable systemd startup (since plasma 5.20)
kwriteconfig5 --file startkderc --group General --key systemdBoot true
