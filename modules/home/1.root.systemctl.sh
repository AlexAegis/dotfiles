#!/bin/sh

echo 'starting and enabling timesyncd service'
systemctl enable --now systemd-timesyncd.service
