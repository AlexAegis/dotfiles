#!/bin/sh

if [ "$mac" ]; then
	export LDFLAGS="-L/usr/local/opt/zlib/lib -L/usr/local/opt/bzip2/lib"
	export CPPFLAGS="-I/usr/local/opt/zlib/include -I/usr/local/opt/bzip2/include"
fi


# Use the install latest plugin, answer no if it asks to reinstall the same
# version
echo n | pyenv install-latest

# Set global version to the latest
pyenv global "$(pyenv versions | tail -n 1 | sed 's/^ *//g' | cut -d ' ' -f 2)"

# Upgrade pip
pip install --upgrade pip
# Install common global tools
pip install setuptools wheel pipenv
# poetry doesn't compile on 3.13
