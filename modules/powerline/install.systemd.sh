#!/bin/sh

# wrapping this isn't needed here as the whole file would be run as root
# anyway but if you'd copy it out and put sudo in the front
# that would only run `echo` as root and not the filewrite
sh -c "echo '
KEYMAP=us
#KEYMAP=hu
FONT=ter-powerline-v16n.psf.gz
' >/etc/vconsole.conf"

# Instant reload
/usr/lib/systemd/systemd-vconsole-setup
