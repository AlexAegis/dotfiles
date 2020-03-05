#!/bin/sh

cp "$HOME"/.config/powerline/fonts/fontconfig/* \
	"$HOME"/.config/fontconfig/conf.d
fc-cache -vf
