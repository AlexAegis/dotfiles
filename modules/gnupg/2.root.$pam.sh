#!/bin/sh

if [ -e /etc/pam.d/gdm ]; then
	appendabsent 'auth     optional  pam_gnupg.so store-only' /etc/pam.d/gdm
	appendabsent 'session  optional  pam_gnupg.so' /etc/pam.d/gdm
fi

if [ -e /etc/pam.d/sddm ]; then
	appendabsent 'auth     optional  pam_gnupg.so store-only' /etc/pam.d/sddm
	appendabsent 'session  optional  pam_gnupg.so' /etc/pam.d/sddm
fi

if [ -e /etc/pam.d/login ]; then
	appendabsent 'auth     optional  pam_gnupg.so store-only' /etc/pam.d/login
	appendabsent 'session  optional  pam_gnupg.so' /etc/pam.d/login
fi
