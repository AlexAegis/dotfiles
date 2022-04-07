#!/bin/sh

# Set up a samba password for the user
smbpasswd -a "${SUDO_USER:-$USER}"
