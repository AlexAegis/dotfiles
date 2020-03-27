#!/bin/sh

systemctl disable --user xsession
systemctl disable --user xbindkeys

rm /etc/X11/xorg.conf.d/??-layout*
