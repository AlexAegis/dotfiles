#!/bin/sh

mkdir -p /etc/pacman.d/hooks/

# Every update of bash would make it symlink itelf to /bin/sh
# This trigger re-symlinks dash to /usr/bin/sh after bash updates
echo "[Trigger]
Operation=Install
Operation=Upgrade
Operation=Remove
Type=Package
Target=nvidia
Target=linux
# Change the linux part above and in the Exec line if a different
# kernel is used

[Action]
Description=Update Nvidia module in initcpio
Depends=mkinitcpio
When=PostTransaction
NeedsTargets
Exec=/bin/sh -c 'while read -r trg; do case \$trg in linux) exit 0; esac \
done; /usr/bin/mkinitcpio -P'
" >/etc/pacman.d/hooks/nvidia.hook
