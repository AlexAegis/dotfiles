#!/bin/sh

# Set up a samba password for the user
smbpasswd -a "${SAMBA_USER:-$USER}"
