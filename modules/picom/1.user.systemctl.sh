#!/bin/sh

# temporary, until it's not merged, use this fork

cd ./resources/picom || exit 1

pacman -Syu --needed --noconfirm meson uthash

meson --buildtype=release . build

ninja -C build

echo "Linking built picom from $(pwd)/build/src/picom"

ln -sf \
	"$(pwd)/build/src/picom" \
	"${HOME}/.local/bin/picom"
