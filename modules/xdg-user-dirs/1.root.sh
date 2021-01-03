#!/bin/sh

# xdg-user-dirs-update errors out with "No default user directories"
# In the 2.users.sh file becuase on some systems /etc/user-dirs.defaults
# is in /etc/xdg/user-dirs.defaults
# https://bugs.debian.org/cgi-bin/bugreport.cgi?bug=647313
# Once this resolves, this script can be reprivileged to user

if [ ! -e /etc/user-dirs.conf ] && [ ! -e /etc/user-dirs.defaults ] \
	&& [ -e /etc/xdg/user-dirs.conf ] && [ -e /etc/xdg/user-dirs.defaults ]
then
	current_pwd=$(pwd)
	cd /etc || exit 1
	ln -s xdg/user-dirs.conf user-dirs.conf
	ln -s xdg/user-dirs.defaults user-dirs.defaults
	cd "$current_pwd" || exit 1
fi
