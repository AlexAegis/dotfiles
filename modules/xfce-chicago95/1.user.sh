#!/bin/sh

CHICAGO95_GIT_DIR="${XDG_DATA_HOME}/chicago95"

if ! [ -d "${CHICAGO95_GIT_DIR}/.git" ]; then
	git clone https://github.com/grassmunk/Chicago95.git "${CHICAGO95_GIT_DIR}"
else
	(
		cd "${CHICAGO95_GIT_DIR}" || exit 1
		git pull
	)
fi

GTK_THEMES_DIR="${XDG_DATA_HOME}/themes"
GTK_ICONS_DIR="${XDG_DATA_HOME}/icons"

mkdir -p "${GTK_THEMES_DIR}"
mkdir -p "${GTK_ICONS_DIR}"

CHICAGO95_THEME_DIR="${GTK_THEMES_DIR}/chicago95"
CHICAGO95_ICONS_DIR="${GTK_ICONS_DIR}/chicago95"
CHICAGO95TUX_ICONS_DIR="${GTK_ICONS_DIR}/chicago95-tux"

rm -rf "${CHICAGO95_THEME_DIR}" # clean up because hidpi modifications can happen
cp -r "${CHICAGO95_GIT_DIR}/Theme/Chicago95" "${CHICAGO95_THEME_DIR}"
cp -r "${CHICAGO95_GIT_DIR}/Icons/Chicago95" "${CHICAGO95_ICONS_DIR}"
cp -r "${CHICAGO95_GIT_DIR}/Icons/Chicago95-tux" "${CHICAGO95TUX_ICONS_DIR}"
cp -r "${CHICAGO95_GIT_DIR}/Extras/override/gtk-3.24/gtk.css" "${XDG_CONFIG_HOME}/gtk-3.0/chicago.css"


# bitmap fonts
cp -r "${CHICAGO95_GIT_DIR}/Fonts/bitmap/cronyx-cyrillic/" "${XDG_DATA_HOME}/fonts/"
cp -r "${CHICAGO95_GIT_DIR}/Fonts/vga_font/MorePerfectDOSVGA.ttf" "${XDG_DATA_HOME}/fonts/"

xfce4-panel-profiles load "${CHICAGO95_GIT_DIR}/Extras/Chicago95_Panel_Preferences.tar.bz2"


# switch to hidpi theme https://github.com/grassmunk/Chicago95/blob/master/INSTALL.md#hidpi
if ! [ -d "${CHICAGO95_THEME_DIR}/xfwm4_lodpi" ]; then
	mv "${CHICAGO95_THEME_DIR}/xfwm4" "${CHICAGO95_THEME_DIR}/xfwm4_lodpi"
	mv "${CHICAGO95_THEME_DIR}/xfwm4_hidpi" "${CHICAGO95_THEME_DIR}/xfwm4"
fi

sed -i 's/#include "button.rc"/include "button.rc"/' "${CHICAGO95_THEME_DIR}/gtk-2.0/gtkrc"

# bigger gtk icons
xfconf-query -c xsettings -p /Gtk/IconSizes -s "gtk-large-toolbar=32,32:gtk-small-toolbar=24,24:gtk-menu=32,32:gtk-dialog=88,88:gtk-button=32,32:gtk-dnd=32,32"

# larger thunar icon size
xfconf-query --create -c thunar -p /shortcuts-icon-size -s "THUNAR_ICON_SIZE_SMALL"
xfconf-query --create -c thunar -p /tree-icon-size -s "THUNAR_ICON_SIZE_SMALLER"

# larger desktop icon size
xfconf-query --create -c xfce4-desktop -p /desktop-icons/icon-size -s 64

cp "${CHICAGO95_GIT_DIR}/Extras/hidpi/.gtkrc-2.0" "$XDG_CONFIG_HOME/gtkrc-2.0"
