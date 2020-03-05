#!/bin/sh

pacman -Syu --needed --noconfirm imwheel

systemctl --user daemon-reload
systemctl --user enable --now imwheel
journalctl --user --unit imwheel
