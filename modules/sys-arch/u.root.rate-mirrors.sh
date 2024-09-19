#!/bin/sh

rate-mirrors --allow-root --protocol https arch | grep -v '^#' | tee /etc/pacman.d/mirrorlist
