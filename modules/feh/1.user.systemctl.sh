#!/bin/sh

systemctl enable --user --now feh-wallpaper.service
systemctl enable --user --now feh-wallpaper.path
systemctl enable --user --now feh-fehbg.path
