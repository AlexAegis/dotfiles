#!/bin/sh

if [ "$mac" ]; then
	export LDFLAGS="-L/usr/local/opt/zlib/lib -L/usr/local/opt/bzip2/lib"
	export CPPFLAGS="-I/usr/local/opt/zlib/include -I/usr/local/opt/bzip2/include"
fi

# update pyenv
(
	cd "$PYENV_HOME" || exit 1
	git pull
)
# shellcheck disable=SC1091
. "./XDG_CONFIG_HOME.python/rc.d/pyenv.sh"

# Use the install latest plugin, answer no if it asks to reinstall the same
# version
echo n | pyenv install-latest

# Set global version to the latest
pyenv versions | tail -n 1 | sed 's/^ *//g' > "$(pyenv root)"/versions

# Upgrade pip
pip install --upgrade pip
# Install common global tools
pip install setuptools wheel pipenv
