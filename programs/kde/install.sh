#!/bin/bash

# Desktop Environment

pacman -Syu plasma kde-applications

# GNOME Integration https://wiki.archlinux.org/index.php/Firefox#KDE/GNOME_integration
pacman -Syu breeze-gtk kde-gtk-config
