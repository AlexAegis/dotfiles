#!/bin/sh

SCRIPT_PATH="$(dirname "$(readlink -f "$0")")"

#shellcheck disable=SC1090
. "$SCRIPT_PATH/0.user.fallback.sh"
#shellcheck disable=SC1090
. "$SCRIPT_PATH/1.user.sh"
