#!/bin/sh

if [ -e "/usr/bin/vi" ]; then
	if [ -e "/usr/bin/vi-bak" ]; then
		rm -f /usr/bin/vi-bak
	fi
	mv /usr/bin/vi /usr/bin/vi-bak
	rm -f /usr/bin/vi
fi

ln -s /usr/bin/vim /usr/bin/vi
