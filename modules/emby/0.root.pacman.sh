#!/bin/sh

emby_version="4.8.0.21"
emby_filename="emby-server-deb_${emby_version}_arm64.deb"
emby_url="https://github.com/MediaBrowser/Emby.Releases/releases/download/${emby_version}/${emby_filename}"
cache_dir="$XDG_CACHE_HOME/emby"
cached_emby_installer="$cache_dir/$emby_filename"


# if it fails you might want to delete the entire cache_dir
# rm -rf "$cache_dir"

distribution=$(grep "^NAME" /etc/os-release 2>/dev/null | grep -oh "=.*" | \
	tr -d '="')

if [ "$distribution" = 'Arch Linux ARM' ]; then
	if [ ! -f "$cached_emby_installer" ]; then
		mkdir -p "$cache_dir"

		wget "$emby_url" -O "$cached_emby_installer"

		paru -Syu --needed --noconfirm debtap
		debtap -u
		(
			cd "$cache_dir" || exit
			echo "Use the name 'emby-server'"
			debtap "$emby_filename"
			pacman -U --needed --noconfirm "${cache_dir}/"*.pkg.tar.zst
		)
	else
		echo "Emby v${emby_version} already installed!"
	fi
else
	pacman -Syu --needed --noconfirm emby-server
fi

