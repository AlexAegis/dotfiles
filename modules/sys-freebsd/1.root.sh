#!/bin/sh

sysrc sshd_enable=YES || exit 1
# Try to enable DHCP for the first alive network adapter
if ! sysrc -s netif -a | grep -q '^ifconfig_'; then
	first_adapter=$(ifconfig | grep -F 'UP,BROADCAST' | cut -d : -f 1 | head -n 1)
	if [ -n "$first_adapter" ]; then
		echo "First online network adapter: $first_adapter"
		sysrc "ifconfig_$first_adapter=DHCP" || exit 1
	else
		echo 'No network adapter up, DHCP not set' >&2
		exit 1
	fi
fi
