#!/bin/bash
# Dependencies: curl

# Presourcing the environment file for the installation
mkdir -p $HOME/.env # Don't let stow manage shared folders
stow -t ~ home-zsh
. ./home-zsh/.config/zsh/.zsh.env
cat ./home-zsh/.config/zsh/.zsh.env
# Addons

# Antibody
# https://getantibody.github.io/

echo Installing Antibody
mkdir -p $ZDOTDIR/tools
curl -sfL git.io/antibody | sh -s - -b $ZDOTDIR/tools

# oh-my-zsh
# https://ohmyz.sh/

# echo Installing oh-my-zsh
# sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" "" --unattended


# stow rc files to home:

# Postlink

antibody bundle < $ZDOTDIR/.zsh-plugins.antibody > $ZDOTDIR/.zsh-plugins.antibody.sh
