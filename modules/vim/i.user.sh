#!/bin/sh

# To prevent symlinking folders
mkdir -p \
	"$XDG_CONFIG_HOME/vim/backup" \
	"$XDG_CONFIG_HOME/vim/bundle" \
	"$XDG_CONFIG_HOME/vim/autoload" \
	"$XDG_CONFIG_HOME/vim/plugin" \
	"$XDG_CONFIG_HOME/vim/colors" \
	"$XDG_CONFIG_HOME/coc/extensions" \
	"$XDG_CONFIG_HOME/nvim"

# Vim will not create these folders for you
# https://vimhelp.org/options.txt.html#%27backupdir%27
mkdir -p \
	"$XDG_CACHE_HOME/vim/swap" \
	"$XDG_CACHE_HOME/tmp" \
	"$XDG_CONFIG_HOME/vim/backup" \
	"$XDG_CONFIG_HOME/vim/undo" \
	"$XDG_CONFIG_HOME/vim/after"
