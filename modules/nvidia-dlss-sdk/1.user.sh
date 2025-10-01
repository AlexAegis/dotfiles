#!/bin/sh

DLSS_SDK_DIR="$XDG_DATA_HOME/DLSS"

if ! [ -d "$DLSS_SDK_DIR/.git" ]; then
	git clone --recurse-submodules https://github.com/NVIDIA/DLSS.git "$DLSS_SDK_DIR"
	git pull
else
	(
		cd "$DLSS_SDK_DIR" || exit 1
		git pull
		git submodule update --init --recursive
	)
fi
