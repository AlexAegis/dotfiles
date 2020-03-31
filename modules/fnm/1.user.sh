#!/bin/sh

# source them for the sake of installation
# shellcheck disable=SC1090
. "$XDG_CONFIG_HOME/environment.d/50-fnm.conf"
. "$XDG_CONFIG_HOME/rc.d/fnm.sh"

fnm install latest
fnm install latest-v13.x
fnm install latest-v12.x
fnm install latest-v10.x
fnm use latest
