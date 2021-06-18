#!/bin/sh

# skip if it's already the default
if ! [ "${SHELL##*\/}" = "zsh" ]; then
	# set the default shell to zsh
	chsh -s /bin/zsh
fi
