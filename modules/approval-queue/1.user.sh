#!/bin/sh

root="$XDG_DATA_HOME/approval-queue"
# Builds in the cache, so the module folder gets no target folder. The service
# and the approval-queue script start the binaries by absolute path, so PATH
# gets the bin folder only to silence the warning of cargo.
PATH="$root/bin:$PATH" cargo install --locked --force \
	--path ./approval-queue \
	--target-dir "$XDG_CACHE_HOME/approval-queue/target" \
	--root "$root"
