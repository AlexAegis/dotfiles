#!/bin/sh

if [ -e "${RUSTUP_HOME:?}" ] && ! [ -w "$RUSTUP_HOME" ]; then
	printf '%s\n' "Skipping rustup: $RUSTUP_HOME is not writable" >&2
	exit 0
fi

# non interactive rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
	| sh -s -- -y --no-modify-path
