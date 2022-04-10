#!/bin/sh

# Is in the extra repository
pacman -Syu --needed --noconfirm arch-install-scripts

# ? Might move into it's own module?
pacman -Syu --needed --noconfirm accountsservice

# 64-bit ARM Arch uses uboot, it's useful to have it's tools installed
if [ "$(uname -m)" = 'aarch64' ]; then
	pacman -Syu --needed --noconfirm uboot-tools
fi
