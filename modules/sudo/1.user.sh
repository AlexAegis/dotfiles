#!/bin/sh

# Install the wheel sudoer file and making sure the 'wheel' group exists
# USER will be expanded here so don't worry it being `root` once executed
if [ ! "$mac" ]; then
	su -c "./resources/install-sudoer ./resources/10-wheel && \
		groupadd --force wheel && \
		usermod -aG wheel $USER"
fi
