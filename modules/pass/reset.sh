#!/bin/sh

# ! Must be run as the user and this is the simplest way to run an entire script
# ! as another user without messing everything up
mkdir -p "$HOME/.password-store"

git config --global credential.helper /usr/bin/pass-git-helper
git config --global credential.useHttpPath true

if [ ! -e "$HOME/.gnupg" ]; then
	# shellcheck disable=SC1090
	. "$HOME/bin/gengpg"
fi

pass init "$(gpg --list-secret-keys |
	grep "^      " |
	tail -1 |
	sed 's/^      //')"

echo "Enter password on the first line then the username on the second."
pass insert --multiline dev/github
