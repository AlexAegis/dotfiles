#!/bin/sh

pacman -S --needed --noconfirm wine
pacman -S --needed --noconfirm winetricks
winetricks tahoma corefonts ie8
