#!/bin/sh

# For the integration with systemd
if command -v systemctl >/dev/null; then
	aura -A --noconfirm openvpn-update-systemd-resolved
fi

# Don't forget to amend the openVPN config
#    script-security 2
#    up /usr/bin/update-systemd-resolved
#    up-restart
#    down /usr/bin/update-systemd-resolved
#    down-pre
