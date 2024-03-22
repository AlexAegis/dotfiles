#!/bin/sh

# wayland support with nvidia cards
appendabsent 'QT_QPA_PLATFORMTHEME="wayland;xcb"' /etc/environment
appendabsent 'GBM_BACKEND=nvidia-drm' /etc/environment
appendabsent '__GLX_VENDOR_LIBRARY_NAME=nvidia' /etc/environment
appendabsent 'ENABLE_VKBASALT=1' /etc/environment
appendabsent 'LIBVA_DRIVER_NAME=nvidia' /etc/environment
