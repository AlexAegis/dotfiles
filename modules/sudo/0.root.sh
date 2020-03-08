#!/bin/sh

# If correct, install
if [ -z "$(sudo visudo -c -q -f ./sudoers.custom)" ]; then
	echo "Installing ./sudoers.custom to /etc/sudoers.d/"
	install --owner=root \
		--group=root \
		--mode=440 ./sudoers.custom /etc/sudoers.d/
else
	echo "Badly formatted sudoers file!"
	exit 1
fi
