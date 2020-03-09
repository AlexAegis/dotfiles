#!/bin/sh

# shellcheck disable=SC1090
. "$HOME/.env.d/fnm.env"
. "$HOME/.rc.d/fnm.sh"

fnm install latest
fnm install latest-v13.x
fnm install latest-v12.x
fnm install latest-v10.x
fnm use latest
