#!/bin/sh

echo system
systemctl disable --now powertop

rm "/etc/systemd/system/powertop.service"
