#!/bin/sh

# Presourcing so it installs to the right location
# shellcheck disable=SC1091
. "./XDG_CONFIG_HOME.rust/environment.d/20-rust.conf"

# ? Shared toolchains: /opt/rustup exists, or its conf link is left
if [ -d /opt/rustup ]; then
	rustup_shared=1
elif [ -L "$XDG_CONFIG_HOME/environment.d/21-rustup-shared.conf" ]; then
	rustup_shared=1
	printf '%s\n' '/opt/rustup is missing, see modules/rust/readme.md' >&2
fi
if [ "${rustup_shared:-}" ]; then
	# shellcheck disable=SC1091
	. "./XDG_CONFIG_HOME.\$rustup_shared.rust/environment.d/21-rustup-shared.conf"
fi
