#!/bin/sh

# Skipping outside a terminal, such as at login
if [ -t 0 ]; then
	GPG_TTY=$(tty)
	export GPG_TTY

	gpg-connect-agent updatestartuptty /bye > /dev/null 2> /dev/null
fi

# SSH_AUTH_SOCK=$(gpgconf --list-dirs agent-ssh-socket)
# export SSH_AUTH_SOCK
