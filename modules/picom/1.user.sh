#!/bin/sh

# temporary, until it's not merged, use this fork

PICOM_DIR="$XDG_CACHE_HOME/picom"

if ! [ -d "$PICOM_DIR/.git" ]; then
	git clone https://github.com/yshui/picom "$PICOM_DIR"
	git checkout next
	git pull
else
	(
		cd "$PICOM_DIR" || exit 1
		git pull
	)
fi

cd "$PICOM_DIR" || exit 1

meson --buildtype=release . build

ninja -C build

echo "Linking built picom from $(pwd)/build/src/picom"

ln -sf \
	"$(pwd)/build/src/picom" \
	"${HOME}/.local/bin/picom"
