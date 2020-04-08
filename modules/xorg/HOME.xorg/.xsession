#!/bin/sh

if [ "$(command -v systemctl 2>/dev/null)" ]; then
	# import DBUS_SESSION_BUS_ADDRESS
	systemctl --user import-environment DBUS_SESSION_BUS_ADDRESS
	# start every user service that has an x target
	systemctl --no-block --user start xsession.target
fi
