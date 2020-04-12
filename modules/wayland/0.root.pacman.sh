#!/bin/sh

pacman -Syu --needed --noconfirm wayland weston

pacman -Syu --needed --noconfirm glfw-wayland glew-wayland
