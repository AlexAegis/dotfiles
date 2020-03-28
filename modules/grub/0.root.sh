#!/bin/sh


# shellcheck disable=SC1090
. "$HOME/bin/loadenv"

xdpyinfo || exit 1

# sets grub resolution to screen resolution
mon_res="$(xdpyinfo | grep -oP 'dimensions:\s+\K\S+')"
# which will be then picked up by arch for the ttys

setval "/etc/default/grub" "GRUB_GFXMODE" "$mon_res"
setval "/etc/default/grub" "GRUB_HIDDEN_TIMEOUT" "0.0"
setval "/etc/default/grub" "GRUB_TIMEOUT" "0.0"

install -Dm 755 "31_hold_shift" "/etc/grub.d/31_hold_shift"

# update bootloader
grub-mkconfig -o /boot/grub/grub.cfg
