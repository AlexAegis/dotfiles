#!/bin/sh

git init --bare "$HOME/.password-store"

git config --global credential.helper /usr/bin/pass-git-helper
git config --global credential.useHttpPath true
