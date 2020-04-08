#!/bin/sh

# TODO: Might have to re-run after kernel update, make a pacman
# hook to automate
yay -Syu --needed --noconfirm polybar
