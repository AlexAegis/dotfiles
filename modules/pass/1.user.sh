#!/bin/sh

ORIGIN="https://AlexAegis@github.com/AlexAegis/.password-store.git" \
	pass-install

git config --global credential.helper /usr/bin/pass-git-helper
