#!/bin/sh

"./XDG_BIN_HOME.sudo/install-sudoer" ./resources/10-wheel

# Making sure the 'wheel' group exists
groupadd --force wheel
# append the 'wheel' group to the user
usermod -aG wheel "${SUDO_USER-$USER}"
