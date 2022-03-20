#!/bin/sh

if ! getent group "autologin" 1>/dev/null; then
	sudo groupadd -r autologin
fi
sudo gpasswd -a "$USER" autologin


sed -i -e "s/^.*autologin-user=.*/autologin-user=$USER/" \
	/etc/lightdm/lightdm.conf
