#!/bin/sh

# making sure it's the syslink if `xdg-user-dirs-update` overwrites it
[ -e "$XDG_CONFIG_HOME/user-dirs.dirs" ] \
	&& rm "$XDG_CONFIG_HOME/user-dirs.dirs"

mkdir -p "$XDG_DESKTOP_DIR" \
		"$XDG_DOCUMENTS_DIR" \
		"$XDG_DOWNLOAD_DIR" \
		"$XDG_MUSIC_DIR" \
		"$XDG_PICTURES_DIR" \
		"$XDG_PUBLICSHARE_DIR" \
		"$XDG_TEMPLATES_DIR" \
		"$XDG_VIDEOS_DIR"

# rm -r ~/Videos ~/Desktop ~/Music ~/Pictures ~/Public ~/Templates ~/Videos
# rm -r "$XDG_DOCUMENTS_DIR" \
# 		"$XDG_DOWNLOAD_DIR" \
# 		"$XDG_MUSIC_DIR" \
# 		"$XDG_PICTURES_DIR" \
# 		"$XDG_PUBLICSHARE_DIR" \
# 		"$XDG_TEMPLATES_DIR" \
# 		"$XDG_VIDEOS_DIR"
#
