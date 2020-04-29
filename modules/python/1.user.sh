#!/bin/sh

# TODO DELETE THIS ONCE THE FALLBACK SCRIPT IS OKAY
mkdir -p $XDG_DATA_HOME/pyenv
git clone https://github.com/pyenv/pyenv.git $XDG_DATA_HOME/pyenv
# TODO DELETE THIS ONCE THE FALLBACK SCRIPT IS OKAY


git clone https://github.com/momo-lab/pyenv-install-latest.git \
	"$(pyenv root)"/plugins/pyenv-install-latest || true

pyenv install-latest
