#!/bin/sh

mkdir -p /etc/systemd/system/getty@tty1.service.d
cp ./resources/getty@tty1.service.d/override-bottom.conf \
	/etc/systemd/system/getty@tty1.service.d/override.conf

# Lightdm return to tty1
mkdir -p /etc/systemd/system/lightdm.service.d/
cp ./resources/override-return.conf \
	/etc/systemd/system/lightdm.service.d/override.conf

systemctl daemon-reload
systemctl restart getty@tty1
