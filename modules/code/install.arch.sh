#!/bin/sh

${SUDO_USER:+sudo -u $SUDO_USER} \
	yay -Syu --needed --noconfirm visual-studio-code-bin
