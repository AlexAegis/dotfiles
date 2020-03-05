#!/bin/sh

mkdir -p "$HOME/.env.d" # Don't let stow manage shared folders
stow -Rt ~ home-pipes
