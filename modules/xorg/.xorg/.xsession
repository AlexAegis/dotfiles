#!/bin/sh

# start every user service that has an x target

if [ "$(command -v systemctl 2>/dev/null)" ]; then
	systemctl --user import-environment PATH DBUS_SESSION_BUS_ADDRESS
	systemctl --no-block --user start xsession.target
fi
