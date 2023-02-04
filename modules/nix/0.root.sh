#!/bin/sh

mkdir -p /nix
# For a single user setup
# chown -R "${SUDO_USER:-$USER}" /nix
# For a multi-user setup
curl -L https://nixos.org/nix/install | sh -- --daemon
