#!/bin/sh

# git submodule --update --recursive

curl -fLo "$XDG_CONFIG_HOME/vim/autoload/plug.vim" --create-dirs \
	https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim

vim +PlugInstall +CocUpdateSync +qall
