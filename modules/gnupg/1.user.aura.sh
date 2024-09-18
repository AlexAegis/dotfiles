#!/bin/sh

# Auto unlock on login
aura -Ayu --noconfirm pam-gnupg
libtool --finish /usr/lib/security
