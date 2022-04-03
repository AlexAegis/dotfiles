#!/bin/sh

STEAMLINK_INSTALL_DIR="$XDG_CACHE_HOME/steamlink"
mkdir "$STEAMLINK_INSTALL_DIR"
wget https://media.steampowered.com/steamlink/rpi/latest/steamlink.deb "$STEAMLINK_INSTALL_DIR"
