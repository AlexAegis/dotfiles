#!/bin/sh

mkdir -p '/etc/sddm.conf.d/'

echo '[X11]
MinimumVT=1' > '/etc/sddm.conf.d/10-tty.conf'

echo '[XDisplay]
DisplayCommand=/usr/share/sddm/scripts/Xsetup

[Theme]
Current=breeze' > '/etc/sddm.conf.d/20-theme.conf'
