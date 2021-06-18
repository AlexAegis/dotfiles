#!/bin/sh

# making sure it's the symlink if `xdg-user-dirs-update` overwrites it
# ? Remember: this step happens before symlinking!
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

