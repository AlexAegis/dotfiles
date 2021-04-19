#!/bin/sh

emerge -u x11-base/xorg-drivers \
	x11-base/xorg-server

"$XDG_BIN_HOME/install-portage-config.sh"
