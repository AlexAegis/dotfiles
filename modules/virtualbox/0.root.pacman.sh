#!/bin/sh

pacman -Syu --needed --noconfirm \
    virtualbox \
    virtualbox-host-modules-arch \
    virtualbox-guest-iso
# For any other kernel than `linux`, like `linux-lts`, use virtualbox-host-dkms
