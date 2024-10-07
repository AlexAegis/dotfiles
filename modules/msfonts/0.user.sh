#!/bin/sh

# aura -Ayu --noconfirm ttf-ms-win11-auto

package_name="ttf-ms-win11-auto"

git_repo_url="https://aur.archlinux.org/${package_name}.git"
install_dir="$XDG_CACHE_HOME/ttf-ms-win11-auto"

if ! [ -d "$install_dir/.git" ]; then
	git clone "$git_repo_url" "$install_dir"
	git pull
else
	(
		cd "$install_dir" || exit 1
		git pull
	)
fi


(
	cd "$install_dir" || exit 1
	rm "${package_name}"*.pkg.tar.zst
	makepkg -si
	sudo pacman -U --noconfirm "${package_name}"*.pkg.tar.zst
)


