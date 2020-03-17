#!/bin/sh

pacman -Syu --needed --noconfirm sddm

# for the slice theme
pacman -Syu --needed --noconfirm qt5-graphicaleffects
