#!/bin/sh

# Is in the extra repository
pacman -Syu --needed --noconfirm arch-install-scripts sys-config

# ? Might move into it's own module?
pacman -Syu --needed --noconfirm accountsservice


