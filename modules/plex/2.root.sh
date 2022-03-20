#!/bin/sh

usermod -a -G transmission plex
usermod -a -G media plex

cp ./resources/nginx.plex.conf "$NGINX_SITES_ENABLED/"
# Installing it globally too
cp ./XDG_BIN_HOME.plex/convert_mkv_to_mp4 /usr/bin/

# shellcheck disable=SC1091
. "./u.root.sh"
