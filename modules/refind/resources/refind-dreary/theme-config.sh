#!/bin/sh

theme_dir=$1

mv "$theme_dir"/highres/* "$theme_dir/"
rm -rf "$theme_dir/clover" \
	"$theme_dir/highres" \
	"$theme_dir/lowres" \
	"$theme_dir/install.sh" \
	"$theme_dir/README.md"
