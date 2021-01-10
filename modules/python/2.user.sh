#!/bin/sh

git clone https://github.com/momo-lab/pyenv-install-latest.git \
	"$(pyenv root)"/plugins/pyenv-install-latest || true

# shellcheck disable=SC1091
. './u.user.sh'
