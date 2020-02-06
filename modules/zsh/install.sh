#!/bin/bash
# Dependencies: curl

# Presourcing the environment file for the installation

stow -t $HOME home-zsh
set -a
. ./home-zsh/.zshenv
set +a

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
# antibody bundle < ./home-zsh/.config/zsh/.zsh-plugins.antibody > $ZDOTDIR/.zsh-plugins.antibody.sh
