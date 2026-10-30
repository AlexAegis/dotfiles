#!/bin/sh

PICOM_DIR="$XDG_CACHE_HOME/picom"

if ! [ -d "$PICOM_DIR/.git" ]; then
	git clone https://github.com/yshui/picom "$PICOM_DIR" || exit 1
else
	(
		cd "$PICOM_DIR" || exit 1
		git pull
	) || exit 1
fi

cd "$PICOM_DIR" || exit 1

git submodule update --init --recursive
meson setup --buildtype=release . build
ninja -C build

echo "Linking built picom from $(pwd)/build/src/picom"

ln -sf \
	"$(pwd)/build/src/picom" \
	"${HOME}/.local/bin/picom"
