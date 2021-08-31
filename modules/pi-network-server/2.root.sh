#!/bin/sh

# Share /var/media with samba
# Samba only has a single configuration file so I'm appending to it.
# This, without a proper ini editor wont allow for easy edits by changing
# this file
# TODO: Refactor with ini editor
mkdir -p /etc/samba
touch /etc/samba/smb.conf
if ! grep -qEi "[PiShare]" /etc/samba/smb.conf >/dev/null; then
cat << EOF >> /etc/samba/smb.conf
[PiShare]
	comment=Pi Share
	path=/var/media
	browseable=yes
	writeable=yes
	only guest=no
	create mask=0740
	directory mask=0750
	public=no
EOF
fi

