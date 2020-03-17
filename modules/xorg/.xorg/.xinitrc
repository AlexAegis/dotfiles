#!/bin/sh

userresources=$HOME/.Xresources
usermodmap=$HOME/.Xmodmap
sysresources=/etc/X11/xinit/.Xresources
sysmodmap=/etc/X11/xinit/.Xmodmap

# merge in defaults and keymaps

if [ -f $sysresources ]; then
	xrdb -merge $sysresources
fi

if [ -f $sysmodmap ]; then
	xmodmap $sysmodmap
fi

if [ -f "$userresources" ]; then

	xrdb -merge "$userresources"

fi

if [ -f "$usermodmap" ]; then
	xmodmap "$usermodmap"
fi

# start some nice programs

if [ -d /etc/X11/xinit/xinitrc.d ]; then
	for f in /etc/X11/xinit/xinitrc.d/?*.sh; do
		# shellcheck disable=SC1090
		[ -x "$f" ] && . "$f"
	done
	unset f
fi

## DESKTOP ENVIRONMENT
# Starting Desktop Environment (Default KDE)
echo "xinit"
# session=${1:-kde}
#
# case $session in
# i3 | i3wm) exec i3 ;;
# kde)
# 	if [ -f ~/.config/kde/kde-bspwm.sh ]; then
# 		# shellcheck disable=SC1090
# 		. ~/.config/kde/kde-bspwm.sh
# 	fi
# 	exec startplasma-x11
# 	;;
# xfce | xfce4) exec startxfce4 ;;
# # No known session, try to run it as command
# *) exec "$1" ;;
# esac
