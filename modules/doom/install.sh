#!/bin/sh

stow -t "$HOME" -d "${0%/*}" .doom

git submodule update

find "${0%/*}" -regex '.*\.7z' -exec 7z x -y -o"$HOME/.config/gzdoom/wad" {} \;
