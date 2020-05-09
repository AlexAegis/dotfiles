#!/bin/sh

# instead of letting it modify startup scripts to source itself...
# shellcheck disable=SC1090
curl -sL https://github.com/shyiko/jabba/raw/master/install.sh | \
	/bin/bash -s -- --skip-rc && . "$JABBA_HOME/jabba.sh"

# ...I symlink the necessary files into my drop-in sourcables
(
	cd "$XDG_CONFIG_HOME/rc.d" || exit 1
	ln -sf "$JABBA_HOME/jabba.sh" jabba.sh
	ln -sf "$JABBA_HOME/jabba.fish" jabba.fish
)
