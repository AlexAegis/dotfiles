#!/bin/sh

# shellcheck disable=SC2154
if [ ! "$pacman" ]; then
	cd ./resources/pass-git-helper/ || exit 1
	python3 setup.py install --user
fi
