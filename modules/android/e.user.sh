#!/bin/sh

# shellcheck disable=SC1091
. "./XDG_CONFIG_HOME.android/environment.d/80-android.conf"
if [ -n "$wsl" ]; then
	. "./XDG_CONFIG_HOME.\$wsl.android/rc.d/80-android.sh"
fi
