#!/bin/sh

root="$XDG_DATA_HOME/agent-open"
# Builds in the cache, so the module folder gets no target folder. The service
# starts the binary by absolute path, so PATH gets the bin folder only to
# silence the warning of cargo.
PATH="$root/bin:$PATH" cargo install --locked --force \
	--path ./agent-open \
	--target-dir "$XDG_CACHE_HOME/agent-open/target" \
	--root "$root"
