#!/bin/sh

# This file runs before stow does
mkdir -p "$XDG_CONFIG_HOME/systemd/user" \
	"$XDG_CONFIG_HOME/environment.d"
