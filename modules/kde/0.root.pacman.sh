#!/bin/sh

# Desktop Environment

pacman -Syu --needed --noconfirm plasma kde-applications

# GNOME Integration
# https://wiki.archlinux.org/index.php/Firefox#KDE/GNOME_integration
pacman -Syu --needed --noconfirm breeze-gtk kde-gtk-config

# Askpass for wallet
pacman -Syu --needed --noconfirm ksshaskpass

# notify-send, for the login environment's notifications
pacman -Syu --needed --noconfirm libnotify

# https://store.kde.org/ integration
pacman -Syu --needed --noconfirm qt6-base qt6-svg qt6-declarative

# Filebrowser, archive manager, disk usage utility, calculator etc
pacman -Syu --needed --noconfirm dolphin ark unrar filelight kcalc gwenview kcolorchooser
