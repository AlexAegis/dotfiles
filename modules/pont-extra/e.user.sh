#!/bin/sh

# pont itself reads this file before doing anything with the module
# to load necessary environment variables.

# shellcheck disable=SC1091
. "./XDG_CONFIG_HOME.pont/environment.d/90-pont.conf"
