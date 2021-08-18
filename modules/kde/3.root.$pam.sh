#!/bin/sh

if [ -e /etc/pam.d/gdm ]; then
	appendabsent 'auth    optional        pam_kwallet5.so' /etc/pam.d/gdm
	appendabsent 'session optional        pam_kwallet5.so  auto_start' /etc/pam.d/gdm
fi

if [ -e /etc/pam.d/gnome-screensaver ]; then
	appendabsent 'auth    optional        pam_kwallet5.so' /etc/pam.d/gnome-screensaver
fi

if [ -e /etc/pam.d/sddm ]; then
	appendabsent 'auth    optional        pam_kwallet5.so' /etc/pam.d/sddm
	appendabsent 'session optional        pam_kwallet5.so  auto_start' /etc/pam.d/sddm
fi

if [ -e /etc/pam.d/passwd ]; then
	appendabsent 'password    optional        pam_kwallet5.so' /etc/pam.d/passwd
fi

if [ -e /etc/pam.d/login ]; then
	appendabsent 'auth       optional     pam_kwallet5.so' /etc/pam.d/login
	appendabsent 'session    optional     pam_kwallet5.so auto_start' /etc/pam.d/login
fi
