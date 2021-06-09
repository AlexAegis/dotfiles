#!/bin/sh

pacman -Syu --needed --noconfirm virtualbox virtualbox-host-modules-arch
# For any other kernel than `linux`, like `linux-lts`, use virtualbox-host-dkms
