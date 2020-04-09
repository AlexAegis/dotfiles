#!/bin/sh

# shellcheck disable=SC1090
. "$XDG_CONFIG_HOME/environment.d/50-python.conf"

echo "PYENV_ROOT $PYENV_ROOT"
echo "pyenv root $(pyenv root)"

git clone https://github.com/momo-lab/pyenv-install-latest.git \
	"$(pyenv root)"/plugins/pyenv-install-latest

pyenv install-latest
