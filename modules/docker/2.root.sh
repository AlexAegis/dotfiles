#!/bin/sh

# Add the current user to the docker group
groupadd --force docker
usermod -aG docker "${SUDO_USER:-$USER}"
