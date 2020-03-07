#!/bin/sh

if [ ! -e "$HOME/.gnupg" ]; then
	echo "Missing GPG keys. Generating."
	# shellcheck disable=SC1091
	. gengpg
fi

if [ ! -e "$HOME/.ssh/id_rsa" ] &&
	[ ! -e "$HOME/.ssh/id_rsa.pub" ]; then
	echo "Missing RSA key. Generating."
	# shellcheck disable=SC1091
	. genrsa
fi

if [ ! -e "$HOME/.ssh/ca.crt" ] &&
	[ ! -e "$HOME/.ssh/ca.key" ] &&
	[ ! -e "$HOME/.ssh/ca.srl" ] &&
	[ ! -e "$HOME/.ssh/server.crt" ] &&
	[ ! -e "$HOME/.ssh/server.csr" ] &&
	[ ! -e "$HOME/.ssh/server.key" ] &&
	[ ! -e "$HOME/.ssh/server.pem" ] &&
	[ ! -e "$HOME/.ssh/client.crt" ] &&
	[ ! -e "$HOME/.ssh/client.csr" ] &&
	[ ! -e "$HOME/.ssh/client.key" ] &&
	[ ! -e "$HOME/.ssh/client.pem" ] &&
	[ ! -e "$HOME/.ssh/client.pfx" ]; then
	echo "Missing certificates. Generating."
	# shellcheck disable=SC1091
	. gencert
fi
