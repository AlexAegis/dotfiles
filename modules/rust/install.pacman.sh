#!/bin/sh

pacman -Syu --needed --noconfirm rustup
rustup default stable
