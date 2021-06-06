#!/bin/sh

# enable sub-pixel RGB rendering globally

mkdir -p /etc/fonts/conf.d
cd /etc/fonts/conf.d || exit 1
ln -sf ../conf.avail/10-sub-pixel-rgb.conf 10-sub-pixel-rgb.conf
