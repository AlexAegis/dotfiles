#!/bin/bash

pacman -Syu --needed --noconfirm pass
${SUDO_USER:+sudo -u $SUDO_USER} \
	yay -Syu --needed --noconfirm pass-git-helper
