#!/bin/sh

mkdir -p /etc/pacman.d/hooks/

# Every update of bash would make it symlink itelf to /bin/sh
# This trigger re-symlinks dash to /usr/bin/sh after bash updates
echo '[Trigger]
Type = Package
Operation = Install
Operation = Upgrade
Target = bash

[Action]
Description = re-symlinking /bin/sh symlink to dash...
When = PostTransaction
Exec = /usr/bin/ln -sfT /usr/bin/dash /usr/bin/sh
Depends = dash' >/etc/pacman.d/hooks/dashbinsh.hook
