# emby

arch install

download deb file for armv7
(read sys-arch for how to install them)
installed, make sure some folders are removed from previous installations IF not installed with pacman!!!!

```sh
# ! If not a pacman installation!
rm /opt/emby-server/
rm /usr/lib/systemd/system/emby-server.service
rm /usr/lib/systemd/system/emby-server@.service
rm /usr/share/doc/emby-server/
```

install it

```sh
pacman -U ./emby-server-4.6.4.0-1-armv7h.pkg.tar.zst
```

create emby user with its group

```sh
useradd -M -U emby
```

optionally add a password to it

```sh
passwd emby
```

and make sure its log folder exists and its assigned to emby:emby

```sh
chown emby:emby /var/lib/emby
```

then enable the server

```sh
systemctl enable --now emby-server
systemctl status emby-server
```
