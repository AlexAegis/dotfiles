#!/bin/sh

# TODO: This is only usable on ubuntu
[ $ubuntu ] && add-apt-repository universe

apt update -y
apt upgrade -y
apt install -y dash git stow tar coreutils
