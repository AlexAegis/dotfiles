#!/bin/sh

git submodule update --init --recursive
# shellcheck source=/dev/null
. "$XDG_CONFIG_HOME/powerline/fonts/install.sh"
