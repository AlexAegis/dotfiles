#!/bin/sh

# until at least 0.8 is in debian/ubuntu, use nightly
neovim_nightly_ppa=neovim-ppa/unstable

if ! grep -q "^deb .*$neovim_nightly_ppa" /etc/apt/sources.list /etc/apt/sources.list.d/*; then
    add-apt-repository "ppa:$neovim_nightly_ppa"
	apt-get update
fi

apt-get install -y neovim
