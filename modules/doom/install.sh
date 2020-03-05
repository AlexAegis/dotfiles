#!/bin/sh

git submodule update --init --recursive

find "${0%/*}" -regex '.*\.7z' -exec 7z x -y -o"$HOME/.config/gzdoom/wad" {} \;
