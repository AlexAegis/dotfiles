#!/bin/sh

# TODO: set GPU_MANUFACTURER env variable to NVIDIA or AMD and install packages
# here based on that

pacman -Syu --needed --noconfirm lib32-nvidia-utils # If using nvidia
pacman -Syu --needed --noconfirm lib32-llvm
pacman -Syu --needed --noconfirm steam
