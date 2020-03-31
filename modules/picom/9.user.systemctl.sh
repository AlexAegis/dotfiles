#!/bin/sh

# temporary, until it's not merged, use Tryone144's fork

cd ./resources/compton || exit 1
git checkout feature/dual_kawase

pacman -Syu --needed --noconfirm meson uthash

meson --buildtype=release . build
ninja -C build
make

systemctl disable --user --now picom
systemctl enable --user --now compton-kawase
