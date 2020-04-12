#!/bin/bash

# Ignore everything after the :
# It's only used to duplicate the same config over multiple instances
# To have a cool glow effect when using blur

BAR_NAME=$(echo "$1" | cut -d ':' -f 1)
polybar -c "$XDG_CONFIG_HOME/polybar/config.ini" "${BAR_NAME:-main}"
