#!/bin/sh

# shellcheck disable=SC1090
. "$XDG_CONFIG_HOME/environment.d/50-fnm.conf"

curl -fsSL https://github.com/Schniz/fnm/raw/master/.ci/install.sh |
	bash -s -- --install-dir "$FNM_DIR" --skip-shell
