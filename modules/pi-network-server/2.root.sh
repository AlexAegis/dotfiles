#!/bin/sh

# Samba only has a single configuration file
mkdir -p /etc/samba
touch /etc/samba/smb.conf

ini-unindent /etc/samba/smb.conf
crudini --merge /etc/samba/smb.conf << EOF
[media]
comment = Media Share
path = /var/media
browseable = yes
writeable = yes
only guest = no
public = no
valid users = @media
force user = media
force group = +media
create mask = 0664
directory mask = 2775
force create mode = 0664
force directory mode = 2775
EOF
ini-indent /etc/samba/smb.conf


mkdir -p /var/media
try_create_service_user.sh media
chown -R media:media /var/media
