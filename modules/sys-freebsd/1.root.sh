#!/bin/sh

sysrc enable sshd
# Try to enable DHCP for the first alive network adapter
if ! grep 'ifconfig_' /etc/rc.conf; then
	first_adapter=$(ifconfig | grep 'UP\,BROADCAST' | cut -d : -f 1)
	echo "First online network adapter: $first_adapter"
	if "$first_adapter"; then
		echo "ifconfig_$first_adapter=\"DHCP\"" >> /etc/rc.conf
	fi
fi
