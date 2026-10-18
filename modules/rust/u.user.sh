#!/bin/sh

if [ -e "${RUSTUP_HOME:?}" ] && ! [ -w "$RUSTUP_HOME" ]; then
	printf '%s\n' "Skipping rustup: $RUSTUP_HOME is not writable" >&2
else
	rustup update
fi
cargo install-update -a
