#!/bin/sh

${SUDO_USER:+sudo -u $SUDO_USER} \
	yay -Syu --needed --noconfirm ly-git
