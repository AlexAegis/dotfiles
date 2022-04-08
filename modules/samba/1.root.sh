#!/bin/sh

# Samba only has a single configuration file
mkdir -p /etc/samba
touch /etc/samba/smb.conf

ini-unindent /etc/samba/smb.conf
crudini --merge /etc/samba/smb.conf << EOF
[global]
security = user
log file = /var/log/samba/%m.log
logging = systemd
EOF
ini-indent /etc/samba/smb.conf
