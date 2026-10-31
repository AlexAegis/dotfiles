#!/bin/sh

RUA_DIR="$XDG_CACHE_HOME/rua"

if ! [ -d "$RUA_DIR/.git" ]; then
	git clone https://aur.archlinux.org/rua.git "$RUA_DIR" || exit 1
else
	(
		cd "$RUA_DIR" || exit 1
		git pull
	) || exit 1
fi

cd "$RUA_DIR" || exit 1
makepkg -si
