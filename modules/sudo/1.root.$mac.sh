#!/bin/sh

# Install the wheel sudoer file and making sure the 'wheel' group exists
# USER will be expanded here so don't worry it being `root` once executed
# macs `su` doesn't default to root, but sudo is preinstalled
./resources/install-sudoer ./resources/10-wheel

dseditgroup -o edit -a "$USER" -t user wheel
