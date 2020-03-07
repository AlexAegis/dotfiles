#!/bin/sh

# sets grub resolution to screen resolution

mon_res="$(xdpyinfo | grep -oP 'dimensions:\s+\K\S+')"

setval "/etc/default/grub" "GRUB_GFXMODE" "$mon_res"
setval "/etc/default/grub" "GRUB_HIDDEN_TIMEOUT" "0.0"
setval "/etc/default/grub" "GRUB_TIMEOUT" "0.0"

install -Dm 755 "31_hold_shift" "/etc/grub.d/31_hold_shift"

# update bootloader
grub-mkconfig -o /boot/grub/grub.cfg
