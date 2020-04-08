#!/bin/sh

# Always clean up
rm -f /etc/X11/xorg.conf.d/??-layout*

# This has to be run as root, but we also need the environment of the
# target user
# loadenv is in the original users home folder
# shellcheck disable=SC1090,SC2016
ORIGINAL_ENVIRONMENT="$(${SUDO_USER:+sudo -Hiu $SUDO_USER} \
	/bin/sh -c '. ~/.profile && env')"

ORIGINAL_XDG_CONFIG_HOME=$(echo "$ORIGINAL_ENVIRONMENT" | \
	grep XDG_CONFIG_HOME= | sed 's/.*=//')
LAYOUT=$(echo "$ORIGINAL_ENVIRONMENT" | \
	grep LAYOUT= | sed 's/.*=//')
LAYOUT_USE_CONF=$(echo "$ORIGINAL_ENVIRONMENT" | \
	grep LAYOUT_USE_CONF= | sed 's/.*=//')

echo "Using $ORIGINAL_XDG_CONFIG_HOME as XDG_CONFIG_HOME."
echo "Installing layout: $LAYOUT. Use conf? $LAYOUT_USE_CONF."

if [ "$LAYOUT" ] && [ "$LAYOUT_USE_CONF" = "1" ]; then
	# prefer the module because the one in home might be just the symlink
	# of the same file in here.
	# But to support local, non version controlled layouts, check there too
	module_file="./XDG_CONFIG_HOME.xorg/xorg/layout/50-layout-$LAYOUT.conf"
	local_file="$ORIGINAL_XDG_CONFIG_HOME/xorg/layout/50-layout-$LAYOUT.conf"
	if [ -e "$module_file" ]; then
		cp "$module_file" /etc/X11/xorg.conf.d/
	elif [ -e "$local_file" ]; then
		cp "$local_file" /etc/X11/xorg.conf.d/
	fi
fi
