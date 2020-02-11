#!/bin/bash

# Enable time sync
systemctl enable systemd-timesyncd.service
systemctl start systemd-timesyncd.service

# Making hard folders
mkdir -p $HOME/.env # Don't let stow manage shared folders
mkdir -p $HOME/.rc # Don't let stow manage shared folders
stow -Rt ~ home
