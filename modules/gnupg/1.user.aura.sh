#!/bin/sh

# Auto unlock on login
aura -A --noconfirm pam-gnupg
libtool --finish /usr/lib/security
