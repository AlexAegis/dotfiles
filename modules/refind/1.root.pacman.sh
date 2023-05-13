#!/bin/sh

mkdir -p /etc/pacman.d/hooks/

# ? This hook runs upgrades refind automatically if refind is installed
install -Dm 755 "./resources/hooks/20-refind-upgrade-local.hook" \
	"/etc/pacman.d/hooks/20-refind-upgrade-local.hook"
install -Dm 755 "./resources/hooks/21-refind-upgrade-mounted.hook" \
	"/etc/pacman.d/hooks/21-refind-upgrade-mounted.hook"
