#!/bin/sh

echo '[X11]
MinimumVT=1' \
	>"/etc/sddm.conf.d/tty.conf"

# install theme https://github.com/RadRussianRus/sddm-slice
# TODO: update this particular submodule
rm -rf /usr/share/sddm/themes/slice
cp -r ./.sddm/.config/sddm/themes/sddm-slice/slice /usr/share/sddm/themes/slice

cp -r ./resources/slice/* /usr/share/sddm/themes/slice
