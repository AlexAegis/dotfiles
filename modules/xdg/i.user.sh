#!/bin/sh

# Load the XDG config defined by the module itself to be used when installing
# shellcheck disable=SC1091
. "./XDG_CONFIG_HOME.xdg/environment.d/10-xdg.conf"
. "./XDG_CONFIG_HOME.xdg/environment.d/11-xdg-user-dirs.conf"

# making sure it's the syslink if `xdg-user-dirs-update` overwrites it
[ -e "$XDG_CONFIG_HOME/user-dirs.dirs" ] \
	&& rm "$XDG_CONFIG_HOME/user-dirs.dirs"

# Creating all the folders
# Since user-dirs.dirs is predefined
# For some reason xdg-user-dirs-update would just
# remove the symlink, and create a new file where every
# user folder points to $HOME
mkdir -p "$XDG_CONFIG_HOME" \
		"$XDG_CACHE_HOME" \
		"$XDG_DATA_HOME" \
		"$XDG_RUNTIME_DIR" \
		"$XDG_BIN_HOME"

mkdir -p "$XDG_DESKTOP_DIR" \
		"$XDG_DOCUMENTS_DIR" \
		"$XDG_DOWNLOAD_DIR" \
		"$XDG_MUSIC_DIR" \
		"$XDG_PICTURES_DIR" \
		"$XDG_PUBLICSHARE_DIR" \
		"$XDG_TEMPLATES_DIR" \
		"$XDG_VIDEOS_DIR"

# Other common folders
mkdir -p "$XDG_CONFIG_HOME/alias.d" \
		"$XDG_CONFIG_HOME/environment.d" \
		"$XDG_CONFIG_HOME/rc.d"

# rm -r ~/Videos ~/Desktop ~/Music ~/Pictures ~/Public ~/Templates ~/Videos
# rm -r "$XDG_DOCUMENTS_DIR" \
# 		"$XDG_DOWNLOAD_DIR" \
# 		"$XDG_MUSIC_DIR" \
# 		"$XDG_PICTURES_DIR" \
# 		"$XDG_PUBLICSHARE_DIR" \
# 		"$XDG_TEMPLATES_DIR" \
# 		"$XDG_VIDEOS_DIR"
#
