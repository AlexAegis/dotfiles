#!/bin/sh

# Always clean up
rm -f /etc/X11/xorg.conf.d/??-layout*

# loadenv is in the original users home folder
# shellcheck disable=SC1090,SC2016
ORIGINAL_ENVIRONMENT="$(${SUDO_USER:+sudo -Hiu $SUDO_USER} \
	/bin/sh -c '. $HOME/bin/loadenv && env')"

ORIGINAL_HOME=$(${SUDO_USER:+sudo -Hiu $SUDO_USER} env | grep HOME= | sed 's/.*=//')
LAYOUT=$(echo "$ORIGINAL_ENVIRONMENT" | grep LAYOUT= | sed 's/.*=//')
LAYOUT_USE_CONF=$(echo "$ORIGINAL_ENVIRONMENT" | grep LAYOUT_USE_CONF= | sed 's/.*=//')

echo "Using $ORIGINAL_HOME as home."
echo "Installing layout: $LAYOUT. Use conf? $LAYOUT_USE_CONF."

if [ "$LAYOUT" ] && [ "$LAYOUT_USE_CONF" = "1" ]; then
	# prefer the module because the one in home might be just the symlink
	# of the same file in here.
	module_file="./.xorg/.config/xorg/layout/50-layout-$LAYOUT.conf"
	local_file="$ORIGINAL_HOME/.config/xorg/layout/50-layout-$LAYOUT.conf"
	if [ -e "$module_file" ]; then
		cp "$module_file" /etc/X11/xorg.conf.d/
	elif [ -e "$local_file" ]; then
		cp "$local_file" /etc/X11/xorg.conf.d/
	fi
fi
