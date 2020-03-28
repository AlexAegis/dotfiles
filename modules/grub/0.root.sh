#!/bin/sh

# exit if x is not available
xdpyinfo || exit 1

# sets grub resolution to the primary screens resolution
# warning, grub only supports a handful of resolutions
# if your monitor has some weird resolution then it will be ignored
primary_res=$(xrandr |  \
grep -oP 'connected primary\s+\K\S+' |  \
grep -Eo '[0-9]*x[0-9]*')

# which will be then picked up by arch for the ttys

setval "/etc/default/grub" "GRUB_GFXMODE" "$primary_res"
setval "/etc/default/grub" "GRUB_HIDDEN_TIMEOUT" "0.0"
setval "/etc/default/grub" "GRUB_TIMEOUT" "0.0"

install -Dm 755 "31_hold_shift" "/etc/grub.d/31_hold_shift"

# update bootloader
grub-mkconfig -o /boot/grub/grub.cfg
