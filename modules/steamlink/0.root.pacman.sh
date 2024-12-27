#!/bin/sh

steamlink_filename="steamlink.deb"
steamlink_url="https://media.steampowered.com/steamlink/rpi/latest/steamlink.deb"
cache_dir="$XDG_CACHE_HOME/steamlink"
cached_steamlink_installer="$cache_dir/$steamlink_filename"

distribution=$(grep "^NAME" /etc/os-release 2>/dev/null | grep -oh "=.*" | \
	tr -d '="')

if [ "$distribution" = 'Arch Linux ARM' ]; then
	if [ ! -f "$cached_steamlink_installer" ]; then
		mkdir -p "$cache_dir"
		wget "$steamlink_url" -O "$cached_steamlink_installer"

		aura -A --noconfirm debtap
		debtap -u
		(
			cd "$cache_dir" || exit
			echo "Use the name 'steamlink'"
			debtap "$steamlink_filename"
			pacman -U --needed --noconfirm "${cache_dir}/"*.pkg.tar.zst
		)
	else
		echo "steamlink already installed!"
	fi
else
    pacman -Syu --needed --noconfirm steamlink
fi

