#!/bin/sh

pacman -Syu --needed --noconfirm python pyenv

git clone https://github.com/momo-lab/pyenv-install-latest.git \
	"$(pyenv root)"/plugins/pyenv-install-latest

pyenv install-latest
