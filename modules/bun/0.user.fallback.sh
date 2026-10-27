#!/bin/sh

# The installer and bun completions can edit the shell startup files, which
# link into this repository
: "${BUN_INSTALL:?}"
scratch=$(mktemp -d) || exit 1
trap 'trap "" HUP INT TERM; rm -rf "$scratch"' EXIT
trap 'trap "" HUP INT TERM; exit 1' HUP INT TERM
curl -fsSL https://bun.sh/install -o "$scratch/install" || exit 1
HOME="$scratch" ZDOTDIR="$scratch" XDG_CONFIG_HOME="$scratch"
export HOME ZDOTDIR XDG_CONFIG_HOME
bash "$scratch/install" || exit 1
SHELL=zsh "$BUN_INSTALL/bin/bun" completions >"$scratch/_bun" || exit 1
mv "$scratch/_bun" "$BUN_INSTALL/_bun"
