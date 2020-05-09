#!/bin/sh

for p in /sys/bus/usb/devices/**/*; do echo "$p"; cat "$p/product"; done
