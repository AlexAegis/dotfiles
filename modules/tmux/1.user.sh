#!/bin/sh

TPM_DIR="$XDG_CONFIG_HOME/tmux/plugins/tpm"


if ! [ -d "$TPM_DIR/.git" ]; then
	mkdir -p "$TPM_DIR"
	git clone https://github.com/tmux-plugins/tpm "$TPM_DIR"
else
	(
		cd "$TPM_DIR" || exit 1
		git pull
	)
fi
