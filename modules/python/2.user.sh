#!/bin/sh

target_folder="$(pyenv root)"/plugins/pyenv-install-latest

if [ ! -d "$target_folder" ]; then
	git clone https://github.com/momo-lab/pyenv-install-latest.git \
		"$target_folder"
else
	(
		cd "$target_folder" || exit 1
		git pull
	)
fi


# shellcheck disable=SC1091
. './u.user.sh'
