#!/bin/sh

# completions

# shellcheck disable=SC1091
if [ -f "${BUN_INSTALL}/_bun" ]; then
	. "${BUN_INSTALL}/_bun"
fi
