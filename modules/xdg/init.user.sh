#!/bin/sh

# Load the XDG config defined by the module itself to be used when installing
# shellcheck disable=SC1091
#set -a
. "./XDG_CONFIG_HOME.xdg/env.d/xdg.env"
# set +a

rm "$XDG_CONFIG_HOME/"
# export XDG_VIDEOS_DIR='/home/alex/vix'
