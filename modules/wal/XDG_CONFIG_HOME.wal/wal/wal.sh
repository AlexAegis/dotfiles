#!/bin/sh

# shellcheck disable=SC1090
if [ -e "$XDG_CONFIG_HOME/rc.d/common.function.sh" ]; then
	. "$XDG_CONFIG_HOME/rc.d/common.function.sh"
fi

if [ ! -e "${WALLPAPER}" ] || [ "${WALLPAPER_FORCE_LAST}" ]; then
	# last_modified_image is defined in the home modules common.function.sh
	WALLPAPER="$(last_modified_image "${WALLPAPERS_DIR}")"
fi

wal --backend haishoku --saturate 0.4 -i "${WALLPAPER}"

# polybar doesn't really want to restart if set wal here, so I poke it
# manually
if [ -e "$XDG_CONFIG_HOME/polybar/config.ini" ]; then
	touch "$XDG_CONFIG_HOME/polybar/config.ini"
fi
