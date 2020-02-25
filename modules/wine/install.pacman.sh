#!/bin/sh

sudo pacman -S --needed --noconfirm wine
sudo pacman -S --needed --noconfirm winetricks
winetricks tahoma corefonts ie8
