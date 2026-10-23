#!/bin/sh

if command -v fnm > /dev/null; then
	_fnm_cache="${XDG_CACHE_HOME:-$HOME/.cache}/loadenv/fnm-env.posix.sh"
	_fnm_link=
	# In a subshell, so a stale link stays out of PATH
	# shellcheck disable=SC1090
	[ -f "$_fnm_cache" ] && [ -r "$_fnm_cache" ] && _fnm_link=$(
		unset FNM_MULTISHELL_PATH
		. "$_fnm_cache" && printf '%s' "${FNM_MULTISHELL_PATH:-}"
	)
	# Refreshed daily, and when a reboot removed its multishell link
	if [ ! -L "$_fnm_link" ] || [ "$(find "$_fnm_cache" -mtime +0 2>/dev/null)" ]; then
		mkdir -p "${_fnm_cache%/*}"
		_fnm_new=$(mktemp "$_fnm_cache.XXXXXX") &&
			fnm env --shell bash > "$_fnm_new" 2>/dev/null &&
			mv -f "$_fnm_new" "$_fnm_cache"
		rm -f "${_fnm_new:-}"
	fi
	# shellcheck disable=SC1090
	[ -f "$_fnm_cache" ] && [ -r "$_fnm_cache" ] && . "$_fnm_cache"
	unset _fnm_cache _fnm_link _fnm_new
fi
