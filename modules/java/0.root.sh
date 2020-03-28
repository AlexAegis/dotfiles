#!/bin/sh

# shellcheck disable=SC1090
. "$HOME/bin/loadenv"

curl -sL https://github.com/shyiko/jabba/raw/master/install.sh | \
	/bin/bash && . "$JABBA_HOME/jabba.sh"
