#!/bin/sh

# Always clean up
sudo rm -f /etc/X11/xorg.conf.d/??-layout*

if [ "$LAYOUT" ] && [ "$LAYOUT_USE_CONF" = "1" ]; then
	# prefer the module because the one in home might be just the symlink
	# of the same file in here.
	# But to support local, non version controlled layouts, check there too
	module_file="./XDG_CONFIG_HOME.xorg/xorg/layout/50-layout-$LAYOUT.conf"
	local_file="$ORIGINAL_XDG_CONFIG_HOME/xorg/layout/50-layout-$LAYOUT.conf"
	if [ -e "$module_file" ]; then
		sudo cp "$module_file" /etc/X11/xorg.conf.d/
	elif [ -e "$local_file" ]; then
		sudo cp "$local_file" /etc/X11/xorg.conf.d/
	fi
fi
