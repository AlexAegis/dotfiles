#!/bin/sh

# This file runs before stow does

mkdir -p "$HOME/.config/systemd/user" \
	"$HOME/.config/environment.d"
