#!/bin/sh

stow -t "$HOME" -d "${0%/*}" .powerline

cp "$HOME"/.config/powerline/fonts/fontconfig/* "$HOME"/.config/fontconfig/conf.d
fc-cache -vf
