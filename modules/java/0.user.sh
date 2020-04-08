#!/bin/sh

# shellcheck disable=SC1090
. "$XDG_CONFIG_HOME/environment.d/50-jabba.conf"

# instead of letting it modify startup scripts to source itself...
curl -sL https://github.com/shyiko/jabba/raw/master/install.sh | \
	/bin/bash -s -- --skip-rc && . "$JABBA_HOME/jabba.sh"

# ...I symlink the necessary files into my drop-in sourcables
(
	cd "$XDG_CONFIG_HOME/rc.d" || exit 1
	ln -s "$JABBA_HOME/jabba.sh" jabba.sh
	ln -s "$JABBA_HOME/jabba.fish" jabba.fish
)
