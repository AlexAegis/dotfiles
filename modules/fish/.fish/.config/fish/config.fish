#!/bin/fish

[ ! -f "$HOME/.profile" ] || fenv source "$HOME/.profile"

fish_vi_key_bindings
