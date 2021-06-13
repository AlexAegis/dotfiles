#!/bin/sh

if [ -d "$NERDFONTS_REPOSITORY/.git" ]; then
	(
		cd "$NERDFONTS_REPOSITORY" || exit 1
		git pull
	)
else
	git clone --depth=1 https://github.com/ryanoasis/nerd-fonts \
		"$NERDFONTS_REPOSITORY"
fi

"$NERDFONTS_REPOSITORY/install.sh"
