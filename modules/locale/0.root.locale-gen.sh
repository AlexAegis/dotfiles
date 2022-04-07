#!/bin/sh

# Enable english and hungarian locales
appendabsent 'en_US.UTF-8 UTF-8' /etc/locale.gen
appendabsent 'hu_HU.UTF-8 UTF-8' /etc/locale.gen

locale-gen

# Set the local as english as the default
# Other tools can later switch between locales
echo 'LANG=en_US.UTF-8
LC_ALL=en_US.UTF-8' > /etc/locale.conf
