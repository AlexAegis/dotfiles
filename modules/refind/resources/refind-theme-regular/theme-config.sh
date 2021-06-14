#!/bin/sh

theme_dir=$1

# delete everything that is overridden
sed /icons_dir/d -i "$theme_dir/theme.conf"
sed /big_icon_size/d -i "$theme_dir/theme.conf"
sed /banner/d -i "$theme_dir/theme.conf"
sed /small_icon_size/d -i "$theme_dir/theme.conf"
sed /selection_big/d -i "$theme_dir/theme.conf"
sed /selection_small/d -i "$theme_dir/theme.conf"
