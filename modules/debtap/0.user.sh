#!/bin/sh

DEBTAP_DIR="$XDG_DATA_HOME/debtap"
pull_status=0

if ! [ -d "$DEBTAP_DIR/.git" ]; then
	git clone https://github.com/helixarch/debtap.git "$DEBTAP_DIR" || exit 1
else
	(
		cd "$DEBTAP_DIR" || exit 1
		git checkout -- debtap || exit 1
		git pull
	)
	pull_status=$?
fi

# temp fix of a bug
sed -i '/You must run at least once/c\echo "temporary bugfix"' \
	"$DEBTAP_DIR/debtap" || exit 1
exit "$pull_status"
