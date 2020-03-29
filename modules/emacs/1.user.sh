#!/bin/sh

# shellcheck disable=SC1090
. "$XDG_CONFIG_HOME/environment.d/emacs.env"

git clone https://github.com/hlissner/doom-emacs "$EMACS_USER_DIRECTORY"
"$EMACS_USER_DIRECTORY/bin/doom" install
