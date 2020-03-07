#!/bin/sh

add-apt-repository universe

apt update
apt upgrade
apt install dash git stow tar coreutils
