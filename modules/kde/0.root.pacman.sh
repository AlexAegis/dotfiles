#!/bin/sh

# Desktop Environment

pacman -Syu --needed --noconfirm plasma kde-applications

# GNOME Integration
# https://wiki.archlinux.org/index.php/Firefox#KDE/GNOME_integration
pacman -Syu --needed --noconfirm breeze-gtk kde-gtk-config

# Askpass for wallet
pacman -Syu --needed --noconfirm ksshaskpass

# https://store.kde.org/ integration
pacman -Syu --needed --noconfirm \
	qt5-base qt5-svg qt5-declarative qt5-quickcontrols
