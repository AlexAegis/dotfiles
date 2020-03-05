#!/bin/sh

pacman -Syu --needed --noconfirm powerline
${SUDO_USER:+sudo -u $SUDO_USER} \
	yay -Syu --needed --noconfirm powerline-fonts-git

setfont /usr/share/kbd/consolefonts/ter-powerline-v16n.psf.gz
