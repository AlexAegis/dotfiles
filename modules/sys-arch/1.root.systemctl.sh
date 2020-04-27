#!/bin/sh

# See bug in readme
# systemctl enable --now numLockOnTty

echo 'starting and enabling timesyncd service'
systemctl enable --now systemd-timesyncd.service
