#!/bin/sh

# start every user service that has an x target
systemctl --user import-environment PATH DBUS_SESSION_BUS_ADDRESS
systemctl --no-block --user start xsession.target
