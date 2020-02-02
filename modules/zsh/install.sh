#!/bin/bash
# Dependencies: curl

# Addons

# Antibody
# https://getantibody.github.io/

echo Installing Antibody
mkdir -p ~/./zsh
curl -sfL git.io/antibody | sh -s - -b $ZDOTDIR/tools/zsh

# oh-my-zsh
# https://ohmyz.sh/

# echo Installing oh-my-zsh
# sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" "" --unattended


# stow rc files to home:

stow -t ~ home-zsh

# Postlink

antibody bundle < $ZDOTDIR/.zsh-plugins.antibody > $ZDOTDIR/.zsh-plugins.antibody.sh
