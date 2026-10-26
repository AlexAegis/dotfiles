#!/bin/zsh

# completions
if [[ -o interactive && -f "${BUN_INSTALL}/_bun" ]]; then
	. "${BUN_INSTALL}/_bun"
fi
