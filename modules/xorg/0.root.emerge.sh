#!/bin/sh

"$XDG_BIN_HOME/install-portage-config.sh"
emerge -u x11-base/xorg-drivers \
	x11-base/xorg-server

