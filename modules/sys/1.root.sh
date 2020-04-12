#!/bin/sh

# enable sub-pixel RGB rendering globally

cd /etc/fonts/conf.d || exit 1
ln -s ../conf.avail/10-sub-pixel-rgb.conf 10-sub-pixel-rgb.conf
