#!/bin/sh

[ -d "$EMACS_USER_DIRECTORY/.git" ] \
	|| git clone https://github.com/hlissner/doom-emacs \
		"$EMACS_USER_DIRECTORY" \
	|| exit 1
"$EMACS_USER_DIRECTORY/bin/doom" install

# TODO: During install it asks for input twice
