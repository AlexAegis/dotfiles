#!/bin/sh

pacman -Syu --needed --noconfirm dash

ln -sfT dash /usr/bin/sh

mkdir -p /etc/pacman.d/hooks/

echo "[Trigger]
Type = Package
Operation = Install
Operation = Upgrade
Target = bash

[Action]
Description = Re-pointing /bin/sh symlink to dash...
When = PostTransaction
Exec = /usr/bin/ln -sfT dash /usr/bin/sh
Depends = dash" >/etc/pacman.d/hooks/dashbinsh.hook
