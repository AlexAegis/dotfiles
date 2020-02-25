#!/bin/bash

sudo pacman -Syu --needed --noconfirm pass
${SUDO_USER:+sudo -u $SUDO_USER} yay -Syu pass-git-helper
