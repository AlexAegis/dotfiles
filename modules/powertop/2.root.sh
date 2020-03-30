#!/bin/sh

cp "./resources/powertop.service" "/etc/systemd/system/"
cp "./resources/usb-power.service" "/etc/udev/rules.d/"

systemctl enable --now powertop
