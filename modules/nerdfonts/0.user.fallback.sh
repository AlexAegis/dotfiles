#!/bin/sh

mkdir -p "$XDG_CACHE_HOME/nerd-fonts"
cd "$XDG_CACHE_HOME/nerd-fonts" || exit 1
if [ -d "$XDG_CACHE_HOME/nerd-fonts/.git" ]; then
	git pull
else
	git clone --depth=1 https://github.com/ryanoasis/nerd-fonts \
		"$XDG_CACHE_HOME/nerd-fonts"
fi

# shellcheck disable=SC1091
. "$XDG_CACHE_HOME/nerd-fonts/install.sh"
