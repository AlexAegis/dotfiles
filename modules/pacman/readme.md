# pacman

> [Arch Wiki](https://wiki.archlinux.org/index.php/pacman)

Contains pacman related config

## Mirrors

Edit the mirrorlist file, and place the closest one on the top

```sh
sudo vim /etc/pacman.d/mirrorlist
```

For example this german one

```ini
## Any
Server = http://ftp.fau.de/archlinux/$repo/os/$arch
```

TODO: Automate the usage of this mirror, but preferrable not by editing the
mirrorlist file
