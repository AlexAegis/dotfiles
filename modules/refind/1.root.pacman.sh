#!/bin/sh

mkdir -p /etc/pacman.d/hooks/

install -Dm 755 "./resources/refind.hook" "/etc/pacman.d/hooks/refind.hook"
