#!/bin/sh

if [ -d "$NERDFONTS_REPOSITORY/.git" ]; then
	(
		cd "$NERDFONTS_REPOSITORY" || exit 1
		git pull
	)
	"$NERDFONTS_REPOSITORY/install.sh"
fi
