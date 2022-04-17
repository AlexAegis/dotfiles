#!/bin/sh

mkdir -p /usr/share/lightdm/lightdm.conf.d
cp "./resources/lightdm-xfce-seat.conf" \
    "/usr/share/lightdm/lightdm.conf.d/lightdm-xfce-seat.conf"
