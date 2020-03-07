#!/bin/sh

mkdir -p "$HOME/.nerd-fonts"
git clone --depth=1 https://github.com/ryanoasis/nerd-fonts \
	"$HOME/.nerd-fonts"

cd "$HOME/.nerd-fonts" || exit 1

# shellcheck disable=SC1091
. "./install.sh"

rm -rf "$HOME/.nerd-fonts"
