#!/bin/sh

# Install the wheel sudoer file and making sure the 'wheel' group exists
# USER will be expanded here so don't worry it being `root` once executed
command="./resources/install-sudoer ./resources/10-wheel && \
		groupadd --force wheel && \
		usermod -aG wheel $USER"

if  [ ! "$mac" ]; then
	if command -v sudo >/dev/null; then
		sudo sh -c "$command"
	else
		su -c "$command"
	fi
fi
