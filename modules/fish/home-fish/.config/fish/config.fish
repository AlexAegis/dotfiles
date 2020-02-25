#!/bin/fish

[ ! -f $HOME/.profile ] || fenv source $HOME/.profile

. $HOME/bin/sourcein.fish "$HOME/.rc/" ".*.[sh|zsh]"

fish_vi_key_bindings
