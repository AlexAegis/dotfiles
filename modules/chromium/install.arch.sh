#!/bin/sh
pacman -S --needed --noconfirm chromium

stow -t ~ home-chromium
