#!/bin/bash

# Install package
apt-get -y install zsh

# Addons

# Antigen
# https://github.com/zsh-users/antigen

echo Installing Antigen
curl -L git.io/antigen > ~/.antigen.zsh


# oh-my-zsh
# https://ohmyz.sh/

echo Installing oh-my-zsh
sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" "" --unattended


# stow rc files to home:

stow -t ~ zsh-home
