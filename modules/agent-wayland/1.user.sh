#!/bin/sh

root="$XDG_DATA_HOME/agent-wayland"
# Builds in the cache, so the module folder gets no target folder. The units
# start the binaries by absolute path, so PATH gets the bin folder only to
# silence the warning of cargo.
PATH="$root/bin:$PATH" cargo install --locked --force \
	--path ./agent-wayland-proxy \
	--target-dir "$XDG_CACHE_HOME/agent-wayland/target" \
	--root "$root"
