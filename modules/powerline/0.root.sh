#!/bin/sh

if ! [ -d "$POWERLINE_DIR/.git" ]; then
	git clone http://www.github.com/powerline/fonts "$POWERLINE_DIR"
else
	(
		cd "$POWERLINE_DIR" || exit 1
		git pull
	)
fi

# shellcheck source=/dev/null
. "$POWERLINE_DIR/install.sh"
