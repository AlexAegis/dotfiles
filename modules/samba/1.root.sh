#!/bin/sh

# Samba only has a single configuration file so I'm appending to it.
# This, without a proper ini editor wont allow for easy edits by changing
# this file
# TODO: Refactor with ini editor
mkdir -p /etc/samba
touch /etc/samba/smb.conf
if ! grep -qEi "\[global\]" /etc/samba/smb.conf >/dev/null; then
cat << EOF >> /etc/samba/smb.conf
[global]
security = user
log file = /var/log/samba/%m.log
logging = systemd
EOF
fi

