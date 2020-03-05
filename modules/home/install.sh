#!/bin/sh

# Enable time sync

if [ "$(command -v systemctl)" ]; then
	echo 'systemd detected'
	echo 'starting and enabling timesyncd service'
	systemctl enable --now systemd-timesyncd.service
fi
