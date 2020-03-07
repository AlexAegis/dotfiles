#!/bin/sh

if [ "$(id -u)" = 0 ]; then
	echo "Can't run with root priviliges!"
	exit 1
fi
