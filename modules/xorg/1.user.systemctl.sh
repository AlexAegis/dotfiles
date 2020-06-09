#!/bin/sh

# echo 'allowed_users=anybody
# needs_root_rights=yes' \
# 	>"/etc/X11/Xwrapper.config"

systemctl start --user autorandr.service
