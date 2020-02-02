#!/bin/sh

# POSIX compliant Defaults sourced from every other shell

# Load environmental variables
. $HOME/scripts/loadenv
# Load aliases
. $HOME/.alias

# TODO: Remove if systemd and SDDM handles this
# if systemctl -q is-active graphical.target && [ ! "$DISPLAY" ] && [ "$XDG_VTNR" -eq 1 ]; then
# 	exec startx
# else
# 	echo not working
# fi
