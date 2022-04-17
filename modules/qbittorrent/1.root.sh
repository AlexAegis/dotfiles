#!/bin/sh

# the user of qbittorrent needs a home folder, it will cache there
try_create_service_user.sh torrent
try_create_service_user.sh media
usermod -aG media torrent
