#!/bin/sh

# Enable systemd startup (since plasma 5.20)
kwriteconfig6 --file startkderc --group General --key systemdBoot true
