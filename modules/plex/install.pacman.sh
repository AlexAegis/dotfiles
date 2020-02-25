${SUDO_USER:+sudo -u $SUDO_USER} yay -Syu plex-media-server

systemctl enable plexmediaserver.service
systemctl start plexmediaserver.service

# https://wiki.archlinux.org/index.php/NTFS-3G

# http://localhost:32400/web/
