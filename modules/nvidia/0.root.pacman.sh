#!/bin/sh

# NVIDIA Driver
# use `nvidia-dkms` for custom kernels,

# TODO: Decide whether to always use the dynamic kernel mode solution
# TODO: Or make this dotmodule autodetect it
# if [ uname -r something something ]
# pacman -Syu --needed --noconfirm dkms nvidia-dkms
# nvidia-utils for vulkan support
pacman -Syu --needed --noconfirm nvidia nvidia-settings nvidia-utils
