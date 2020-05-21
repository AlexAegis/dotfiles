#!/bin/sh

systemctl disable --user --now feh-wallpaper.service
systemctl disable --user --now feh-wallpaper.path
systemctl disable --user --now feh-fehbg.path
