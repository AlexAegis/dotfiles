#!/bin/bash

if [ ! -x /opt/homebrew/bin/brew ]; then
	/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
fi
