#!/bin/sh

GPG_TTY=$(tty)
export GPG_TTY

gpg-connect-agent updatestartuptty /bye >/dev/null

SSH_AUTH_SOCK=$(gpgconf --list-dirs agent-ssh-socket)
export SSH_AUTH_SOCK
