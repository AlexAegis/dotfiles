#!/bin/sh

if command -v fnm > /dev/null; then
	_fnm_cache="${XDG_CACHE_HOME:-$HOME/.cache}/loadenv/fnm-env.sh"
	if [ ! -f "$_fnm_cache" ] || [ "$(find "$_fnm_cache" -mtime +1 2>/dev/null)" ]; then
		mkdir -p "${_fnm_cache%/*}"
		fnm env > "$_fnm_cache" 2>/dev/null
	fi
	# shellcheck disable=SC1090
	. "$_fnm_cache"
fi
