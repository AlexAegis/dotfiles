#!/bin/sh

cp "$XDG_CONFIG_HOME"/powerline/fonts/fontconfig/* \
	"$XDG_CONFIG_HOME"/fontconfig/conf.d
fc-cache -vf
