#!/bin/sh

# Add the current user to the docker group
usermod -aG docker "${SUDO_USER:-$USER}"
