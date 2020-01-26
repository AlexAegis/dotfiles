#!/bin/bash

# Install package
apt-get -y install zsh

# Addons

# Antibody
# https://getantibody.github.io/

echo Installing Antibody
mkdir ~/tools 2>&/dev/null
mkdir ~/tools/zsh 2>&/dev/null
curl -sfL git.io/antibody | sh -s - -b ~/tools/zsh

# oh-my-zsh
# https://ohmyz.sh/

echo Installing oh-my-zsh
sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" "" --unattended


# stow rc files to home:

stow -t ~ home-zsh

# Postlink

antibody bundle < ~/.zsh-plugins.antibody > ~/.zsh-plugins.antibody.sh
