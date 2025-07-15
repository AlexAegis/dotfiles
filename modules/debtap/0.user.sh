#!/bin/sh

DEBTAP_DIR="$XDG_DATA_HOME/debtap"

if ! [ -d "$DEBTAP_DIR/.git" ]; then
	git clone https://github.com/helixarch/debtap.git "$DEBTAP_DIR"
else
	(
		cd "$DEBTAP_DIR" || exit 1
		git pull
	)
fi

# temp fix of a bug
sed -i '/You must run at least once/c\echo "temporary bugfix"' "$DEBTAP_DIR/debtap"
