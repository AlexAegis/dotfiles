#!/usr/bin/env sh

# TODO: set GPU_MANUFACTURER env variable to NVIDIA or AMD and install packages here based on that

sudo pacman -Syu --needed --noconfirm lib32-nvidia-utils # If using nvidia
sudo pacman -Syu --needed --noconfirm lib32-llvm
sudo pacman -Syu --needed --noconfirm steam
