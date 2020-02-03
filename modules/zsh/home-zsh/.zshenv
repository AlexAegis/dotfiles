#!/usr/bin/zsh

# Common environment TODO handled by .zlogin
# . $HOME/.profile

# ZSH Environment
ZDOTDIR="$HOME/.config/zsh"
PATH="$PATH:$ZDOTDIR/tools/"

# Disable oh my zsh autoupdate because it's handled by antibody
# And also setting a ZSH variable for oh my zsh to work properly
# https://github.com/getantibody/antibody/issues/218
DISABLE_AUTO_UPDATE=true
ZSH="${HOME}/.cache/antibody/https-COLON--SLASH--SLASH-github.com-SLASH-robbyrussell-SLASH-oh-my-zsh"
