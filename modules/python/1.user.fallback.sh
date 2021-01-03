#!/bin/sh

PYENV_DIR="$XDG_DATA_HOME/pyenv"

if [ -d "$PYENV_DIR/.git" ]; then
	(
		cd "$PYENV_DIR" || exit 1
		git pull
	)
else
	mkdir -p "$PYENV_DIR"
	git clone https://github.com/pyenv/pyenv.git "$PYENV_DIR"
fi
