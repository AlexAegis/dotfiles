#!/bin/sh

git clone https://github.com/hlissner/doom-emacs "$EMACS_USER_DIRECTORY"
"$EMACS_USER_DIRECTORY/bin/doom" install

# TODO: During install it asks for input twice
