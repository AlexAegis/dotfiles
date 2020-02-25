#!/bin/sh

sudo pacman -S wine
sudo pacman -S winetricks
winetricks tahoma corefonts ie8
