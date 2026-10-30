#!/bin/sh

# KDE writes its own schemes there, so stow must not fold it into a link
mkdir -p "$XDG_DATA_HOME/color-schemes"
