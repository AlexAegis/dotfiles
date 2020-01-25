#!/bin/bash

# Setup default repositories for zypper.

# https://en.opensuse.org/Package_repositories
zypper -n ar -cfp 90 http://ftp.gwdg.de/pub/linux/misc/packman/suse/openSUSE_Tumbleweed/ packman
zypper -n refresh
