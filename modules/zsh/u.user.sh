#!/bin/sh

echo "Updating plugins"

# Antidote https://getantidote.github.io/
if ! [ -d "$ANTIDOTE_DIR/.git" ]; then
	echo "Installing Antidote..."
	git clone --depth=1 https://github.com/mattmc3/antidote.git \
		"$ANTIDOTE_DIR" || exit 1
else
	echo "Updating Antidote..."
	(
		cd "$ANTIDOTE_DIR" || exit 1
		git pull
	) || exit 1
fi

/bin/zsh "./XDG_BIN_HOME.zsh/antidote-bundle.zsh"

# antidote update
