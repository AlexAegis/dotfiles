#!/bin/sh

YAY_DIR="$XDG_CACHE_HOME/yay"

if ! [ -d "$YAY_DIR/.git" ]; then
	git clone https://aur.archlinux.org/paru.git "$YAY_DIR"
else
	(
		cd "$YAY_DIR" || exit 1
		git pull
	)
fi

cd "$YAY_DIR" || exit 1
makepkg --noconfirm --needed -si
