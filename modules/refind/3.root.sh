#!/bin/sh

# Install config

if [ -d "/boot/EFI/refind" ]; then
	if [ -e "./resources/refind.$(hostname).conf" ]; then
		install -Dm 755 "./resources/refind.$(hostname).conf" \
			"/boot/EFI/refind/refind.conf"
	else
		echo "No config file found for this host!"
		exit 1
	fi
else
	echo "No refind folder found. Is your efi in /boot/EFI?"
	exit 1
fi

# Install themes
rm -rf "/boot/EFI/refind/themes"
cp -r "./resources/themes" "/boot/EFI/refind"
chmod -R 755 "/boot/EFI/refind/themes"
