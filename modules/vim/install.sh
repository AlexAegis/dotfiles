#!/bin/sh

stow -d "$SCRIPT_PATH" -t "$HOME" .vim

# git submodule --update --recursive

curl -fLo "$HOME/.vim/autoload/plug.vim" --create-dirs \
	https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim
