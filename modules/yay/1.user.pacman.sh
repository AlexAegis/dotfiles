#!/bin/sh

# TODO: update submodule

cd "./resources/yay" || exit 1
makepkg --noconfirm --needed -si
