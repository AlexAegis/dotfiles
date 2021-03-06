#!/bin/sh

# Install Android SDK CLI
# Latest at https://developer.android.com/studio/#downloads
latest_sdk_revision="6858069"
cache_location="$XDG_CACHE_HOME/commandlinetools-linux-${latest_sdk_revision}_latest.zip"
if [ ! -f "$cache_location" ]; then
	wget https://dl.google.com/android/repository/commandlinetools-linux-${latest_sdk_revision}_latest.zip -O "$cache_location"
	mkdir -p "$ANDROID_HOME"
	unzip -o "$cache_location" -d "$ANDROID_HOME"
fi

# shellcheck disable=SC1091
. "./u.user.sh"
