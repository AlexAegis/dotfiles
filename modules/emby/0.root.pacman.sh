#!/bin/sh

emby_fallback_version="4.9.5.0"
emby_releases_api="https://api.github.com/repos/MediaBrowser/Emby.Releases/releases/latest"

# the latest endpoint never resolves to a prerelease
emby_version=$(curl -sf --max-time 15 "$emby_releases_api" |
	sed -n 's/.*"tag_name": *"\([^"]*\)".*/\1/p' | head -1)

if [ -z "$emby_version" ]; then
	emby_version="$emby_fallback_version"
	echo "Could not reach the Emby release api, falling back to v${emby_version}"
fi

installed_version=$(pacman -Q emby-server 2>/dev/null | cut -d' ' -f2 | \
	cut -d- -f1)

if [ "$installed_version" = "$emby_version" ]; then
	echo "Emby v${emby_version} already installed!"
	exit 0
fi

emby_filename="emby-server-deb_${emby_version}_arm64.deb"
emby_url="https://github.com/MediaBrowser/Emby.Releases/releases/download/${emby_version}/${emby_filename}"
cache_dir="$XDG_CACHE_HOME/emby"
cached_emby_installer="$cache_dir/$emby_filename"


# if it fails you might want to delete the entire cache_dir
# rm -rf "$cache_dir"

distribution=$(grep "^NAME" /etc/os-release 2>/dev/null | grep -oh "=.*" | \
	tr -d '="')

if [ "$distribution" = 'Arch Linux ARM' ] || [ "$distribution" = 'Manjaro ARM' ]; then
	mkdir -p "$cache_dir"

	if [ ! -f "$cached_emby_installer" ]; then
		wget "$emby_url" -O "$cached_emby_installer"
	fi

	(
		cd "$cache_dir" || exit
		# -Q so an unattended run does not stop on debtap's prompts
		debtap -Q "$emby_filename"
		# debtap writes every install hook with a lone comment as its body,
		# which bash refuses to parse, and all of them are empty anyway
		pacman -U --needed --noconfirm --noscriptlet \
			"${cache_dir}/emby-server-${emby_version}-"*.pkg.tar.zst
	)

	find "$cache_dir" -type f ! -name "*${emby_version}*" -delete
else
	pacman -Syu --needed --noconfirm emby-server
fi
