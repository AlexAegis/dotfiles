#!/bin/sh

if [ -d "$NERDFONTS_REPOSITORY/.git" ]; then
	# shellcheck source=/dev/null
	. './u.user.sh'
else
	git clone --depth=1 https://github.com/ryanoasis/nerd-fonts \
		"$NERDFONTS_REPOSITORY"
	"$NERDFONTS_REPOSITORY/install.sh"
fi
