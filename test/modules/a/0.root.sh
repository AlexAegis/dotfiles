#!/bin/sh

if [ "$(id -u)" != 0 ]; then
	echo "Can't run without root priviliges!"
	exit 1
fi
