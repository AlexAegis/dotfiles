#!/bin/sh

# Make sure the emby user and group exists
try_create_service_user.sh emby
try_create_service_user.sh media
try_create_service_user.sh torrent
usermod -aG media emby
usermod -aG torrent emby
# Link emby to /var/media
./XDG_BIN_HOME.emby/link_emby_database.sh /var/media/emby
