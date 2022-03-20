#!/bin/sh

# Share /var/media with samba
# Samba only has a single configuration file so I'm appending to it.
# This, without a proper ini editor wont allow for easy edits by changing
# this file
# TODO: Refactor with ini editor
mkdir -p /etc/samba
touch /etc/samba/smb.conf
if ! grep -qEi "\[pimedia\]" /etc/samba/smb.conf >/dev/null; then
cat << EOF >> /etc/samba/smb.conf
[media]
	comment=Media Share
	path=/var/media
	browseable=yes
	writeable=yes
	only guest=no
	public=no
	valid users = @media
	force user = media
	force group = +media
	create mask = 0664
	directory mask = 2775
	force create mode = 0664
	force directory mode = 2775
EOF
fi

mkdir -p /var/media
useradd -M -U media
chown -R media:media /var/media
