#!/bin/zsh

# shellcheck disable=SC1091
. "$ANTIDOTE_DIR/antidote.zsh"

antidote bundle < "$ZDOTDIR/.zsh-plugins.txt" > "$ZDOTDIR/.zsh-plugins.zsh"
