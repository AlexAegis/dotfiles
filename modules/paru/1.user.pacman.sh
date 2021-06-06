#!/bin/sh

PARU_DIR="$XDG_CACHE_HOME/paru"

if ! [ -d "$PARU_DIR/.git" ]; then
	git clone https://aur.archlinux.org/paru.git "$PARU_DIR"
else
	(
		cd "$PARU_DIR" || exit 1
		git pull
	)
fi

cd "$PARU_DIR" || exit 1
makepkg --noconfirm --needed -si
