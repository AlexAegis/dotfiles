#!/bin/sh

mkdir -p '/etc/sddm.conf.d/'

echo '[X11]
MinimumVT=1' > '/etc/sddm.conf.d/10-tty.conf'

# echo '[Theme]
# Current=slice' > '/etc/sddm.conf.d/20-theme.conf'

echo '[Theme]
Current=chili' > '/etc/sddm.conf.d/20-theme.conf'

# install theme https://github.com/RadRussianRus/sddm-slice
# TODO: update this particular submodule
rm -rf /usr/share/sddm/themes/slice
rm -rf /usr/share/sddm/themes/sddm-chili

# cp -r ./XDG_CONFIG_HOME.sddm/sddm/themes/sddm-slice/slice \
# 	/usr/share/sddm/themes/slice

cp -r ./resources/sddm-chili \
	/usr/share/sddm/themes/chili

# cp -r ./resources/slice/* /usr/share/sddm/themes/slice

cp -r ./resources/sddm-chili-user/* /usr/share/sddm/themes/chili/

