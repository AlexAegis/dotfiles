#!/bin/sh

# Define the media share. Done without crudini on purpose: crudini is not
# guaranteed to be installed on the server, and an idempotent append keeps the
# share reproducible regardless. Files are forced to the `media` user/group so
# clients can read and write the library without needing that membership.

if ! grep -q "^\[media\]" /etc/samba/smb.conf; then
	cat >> /etc/samba/smb.conf << 'EOF'

[media]
	path = /var/media
	browseable = yes
	writable = yes
	valid users = alex
	force user = media
	force group = media
	create mask = 0664
	directory mask = 0775
EOF
fi
