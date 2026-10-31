#!/bin/sh

YAY_DIR="$XDG_CACHE_HOME/yay"

if ! [ -d "$YAY_DIR/.git" ]; then
	git clone https://aur.archlinux.org/yay.git "$YAY_DIR" || exit 1
else
	if [ "$(git -C "$YAY_DIR" config --get remote.origin.url)" != https://aur.archlinux.org/yay.git ]; then
		printf '%s\n' "Not a clone of yay.git, remove it: $YAY_DIR" >&2
		exit 1
	fi
	(
		cd "$YAY_DIR" || exit 1
		git pull
	) || exit 1
fi

cd "$YAY_DIR" || exit 1
makepkg --noconfirm --needed -si
