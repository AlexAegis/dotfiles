#!/bin/sh

groupadd --force media
# Create media directory
mkdir -p /var/media
chown root:media /var/media
# Add your user to the group for access
usermod -aG media "${SUDO_USER:-USER}"
