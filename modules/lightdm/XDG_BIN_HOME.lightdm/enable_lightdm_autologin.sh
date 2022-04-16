#!/bin/sh

username="${SUDO_USER:-$USER}"

if ! getent group "autologin" 1>/dev/null; then
	sudo groupadd -r autologin
fi
sudo gpasswd -a "$username" autologin

sudo sed -i -e "s/^.*autologin-user=.*/autologin-user=$username/" \
	/etc/lightdm/lightdm.conf
