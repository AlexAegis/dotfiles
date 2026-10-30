#!/bin/sh

# Builds in the cache, so the module folder gets no target folder
cargo install --locked --force \
	--path ./agent-wayland-proxy \
	--target-dir "$XDG_CACHE_HOME/agent-wayland/target" \
	--root "$XDG_DATA_HOME/agent-wayland"
