#!/bin/sh

# TODO: This is only usable on ubuntu
# shellcheck disable=SC2154
[ "$ubuntu" ] && add-apt-repository universe

apt update -y
apt upgrade -y
apt install -y dash git stow tar coreutils
