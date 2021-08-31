#!/bin/sh

# A unique key could be generated but that would but only the default key could
# be opened with PAM to avoid entering a password
if [ ! -e "$HOME/.ssh/pi_rsa" ]; then
	genrsa "pi_rsa"
fi

# Assuming the target user is the same as the current one
ssh-copy-id -i "$HOME/.ssh/pi_rsa" pi
