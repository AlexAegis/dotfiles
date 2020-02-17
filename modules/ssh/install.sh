#!/bin/sh

mkdir -p "$HOME/.ssh"
stow -t ~ home-ssh
