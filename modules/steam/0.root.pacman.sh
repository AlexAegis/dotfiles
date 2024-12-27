#!/bin/sh

steam_filename="steam.deb"
steam_url="https://cdn.cloudflare.steamstatic.com/client/installer/steam.deb"
cache_dir="$XDG_CACHE_HOME/steam"
cached_steam_installer="$cache_dir/$steam_filename"

distribution=$(grep "^NAME" /etc/os-release 2>/dev/null | grep -oh "=.*" | \
	tr -d '="')

if [ "$distribution" = 'Arch Linux ARM' ]; then
    # ! Doesn't work. Unmet dependencies!
	if [ ! -f "$cached_steam_installer" ]; then
		mkdir -p "$cache_dir"
		wget "$steam_url" -O "$cached_steam_installer"

		aura -A --noconfirm debtap
		debtap -u
		(
			cd "$cache_dir" || exit
			echo "Use the name 'steam'"
			debtap "$steam_filename"
			pacman -U --needed --noconfirm "${cache_dir}/"*.pkg.tar.zst
		)
	else
		echo "Steam already installed!"
	fi
else
	# TODO: set GPU_MANUFACTURER env variable to NVIDIA or AMD and install packages
    # here based on that
    pacman -Syu --needed --noconfirm lib32-nvidia-utils # If using nvidia
    pacman -Syu --needed --noconfirm lib32-llvm
    pacman -Syu --needed --noconfirm steam
fi

