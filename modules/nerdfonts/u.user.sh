#!/bin/sh

if ! command -v pacman; then
	if [ -d "$NERDFONTS_REPOSITORY/.git" ]; then
		(
			cd "$NERDFONTS_REPOSITORY" || exit 1
			hash_before=$(git rev-parse HEAD)
			git pull
			hash_after=$(git rev-parse HEAD)
			if [ "$hash_before" != "$hash_after" ]; then
				"./install.sh"
			fi
		)
	fi
fi
