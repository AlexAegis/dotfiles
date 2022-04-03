#!/bin/sh

emby_version="4.7.0.32"
emby_url="https://github.com/MediaBrowser/Emby.Releases/releases/download/${emby_version}/emby-server-snap_${emby_version}_arm64.snap"
cache_location="$XDG_CACHE_HOME/emby/emby-server-snap_${emby_version}_arm64.snap"

architecture=$(uname -m)
if [ "${architecture}" != "${architecture#arm}" ]; then
	if [ ! -f "$cache_location" ]; then
		wget "$emby_url" -O "$cache_location"
	fi


else
	pacman -Syu --needed --noconfirm emby-server
fi

