#!/bin/bash
# Dependencies: curl

# Presourcing the environment file for the installation

stow -t $HOME home-zsh
set -a
. ./home-zsh/.zshenv
set +a

# Addons

# Antibody https://getantibody.github.io/

echo Installing Antibody
mkdir -p $ZDOTDIR/tools
curl -sfL git.io/antibody | sh -s - -b $ZDOTDIR/tools

echo Installing plugins
antibody bundle < $ZDOTDIR/.zsh-plugins.antibody > $ZDOTDIR/.zsh-plugins.antibody.sh

echo Updating plugins
antibody update
