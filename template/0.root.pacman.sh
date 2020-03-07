#!/bin/sh

# When installing with pacman always include the --needed and --noconfirm flags
pacman -Syu --needed --noconfirm PACKAGE

# When installing with yay you shoud never use sudo.
# This will sudo back to the user of the original sudo command.
${SUDO_USER:+sudo -u $SUDO_USER} \
	yay -Syu --needed --noconfirm PACKAGE
