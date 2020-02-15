#!/bin/fish

[ ! -f $HOME/.profile ] || fenv source $HOME/.profile



. $HOME/bin/sourcein "$HOME/.rc/" ".*.[sh|zsh]"
