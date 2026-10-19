#!/bin/sh

# The fonts come from 0.root.sh
[ "${PONT_ROOT_FLAG:-1}" = 1 ] || exit 0
cp "$POWERLINE_DIR"/fontconfig/* \
	"$XDG_CONFIG_HOME"/fontconfig/conf.d
