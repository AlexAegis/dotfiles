#!/bin/sh

pacman -Syu --needed --noconfirm imwheel

mkdir -p "$HOME/.config"
stow -t ~ home-imwheel

systemctl --user daemon-reload
systemctl --user enable --now imwheel
journalctl --user --unit imwheel
