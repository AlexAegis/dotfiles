#!/bin/sh

curl -fsSL https://github.com/Schniz/fnm/raw/master/.ci/install.sh |
	bash -s -- --install-dir "$FNM_DIR" --skip-shell
