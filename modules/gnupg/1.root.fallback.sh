#!/bin/sh

pam_gnupg_git="$XDG_CACHE_HOME/pam-gnupg-git"

if ! [ -d "$pam_gnupg_git/.git" ]; then
	git clone https://www.github.com/cruegge/pam-gnupg "$pam_gnupg_git"
else
	(
		cd "$pam_gnupg_git" || exit 1
		git pull
	)
fi

cd "$pam_gnupg_git" || exit 1

./autogen.sh

if [ -d /lib/x86_64-linux-gnu/security ]; then
	# Ubuntu has it's pam *.so files here
	./configure --with-moduledir=/lib/x86_64-linux-gnu/security
else
	# Defaults to /lib/security
	./configure
fi

make
make install
