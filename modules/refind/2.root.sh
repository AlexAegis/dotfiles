#!/bin/sh

if [ -z "$REFIND_ROOT" ]; then
	if [ -d /mnt/refind/EFI/refind ]; then
		REFIND_ROOT="/mnt/refind"
	elif [ -d /boot/EFI/refind ]; then
		REFIND_ROOT="/boot"
	fi
fi

if [ -n "$REFIND_ROOT" ]; then
	echo "Using Refind root at $REFIND_ROOT"
	# update refind
	refind-install --yes --root "$REFIND_ROOT"
	# add custom config file
	echo Copying resources to "$REFIND_ROOT"
	cp -r resources/boot/* "$REFIND_ROOT"
else
	echo "No refind installation detected, mount it under /boot/EFI or " \
		"/mnt/refind/boot/EFI"
fi

