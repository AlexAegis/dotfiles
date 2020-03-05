#!/bin/fish

[ ! -f "$HOME/.profile" ] || fenv source "$HOME/.profile"

# TODO: Remove sourcein
source "$HOME/bin/sourcein.fish" "$HOME/.rc/" ".*.[sh|fish]"

fish_vi_key_bindings
