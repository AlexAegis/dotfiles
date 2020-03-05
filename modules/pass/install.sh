#!/bin/sh

# TODO: Decide if needed
# git init --bare "$HOME/.password-store"

git config --global credential.helper /usr/bin/pass-git-helper
git config --global credential.useHttpPath true

pass init "$(gpg --list-secret-keys | grep "^      " | tail -1)"
