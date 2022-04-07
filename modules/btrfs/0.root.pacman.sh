#!/bin/sh

# reiserfsprogs is a missing dependency for btrfs-convert
pacman -Syu --needed --noconfirm btrfs-progs reiserfsprogs
