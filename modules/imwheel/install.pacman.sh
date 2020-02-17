#!/bin/sh

pacman -Syu imwheel

mkdir -p "$HOME/.config"
stow -t ~ home-imwheel

systemctl --user daemon-reload
systemctl --user enable --now imwheel
journalctl --user --unit imwheel
