#!/bin/bash

# Desktop Environment

pacman -Syu plasma kde-applications

# GNOME Integration https://wiki.archlinux.org/index.php/Firefox#KDE/GNOME_integration
pacman -Syu breeze-gtk kde-gtk-config

# Askpass for wallet
pacman -Syu ksshaskpass

ENV="export GIT_ASKPASS='/usr/bin/ksshaskpass'"


grep -qF -- ".env" "$2" || echo "$1" >> "$2"

# https://store.kde.org/ integration
sudo pacman -S qt5-base qt5-svg qt5-declarative qt5-quickcontrols
yay -S ocs-url
