#!/bin/sh

# Use the install latest plugin, answer no if it asks to reinstall the same
# version
echo n | pyenv install-latest

# Set global version to the latest
pyenv versions | tail -n 1 | sed 's/^ *//g' > "$(pyenv root)"/version

# Upgrade pip
pip install --upgrade pip
# Install common global tools
pip install setuptools wheel
