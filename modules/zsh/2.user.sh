#!/bin/sh

# it's already the default on macOS
if ! [ "$mac" ]; then
	# set the default shell to zsh
	chsh -s /bin/zsh
fi
