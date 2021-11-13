#!/bin/fish

[ ! -f "$HOME/.profile" ] || fenv source "$HOME/.profile"

fish_vi_key_bindings

[ -s "/home/alex/.local/share/jabba/jabba.fish" ]; and source "/home/alex/.local/share/jabba/jabba.fish"
