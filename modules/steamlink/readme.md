# [Steam Link](https://store.steampowered.com/remoteplay#anywhere)

## Debian Package

## Arch

Use [debtap](../debtap) to repackage the debian package and install that.
But the package you install is just a wrapper and it itself depends on
`apt-get`.

## Raspberry Pi

There is an [AUR package](https://aur.archlinux.org/packages/steamlink-raspberrypi/)
but it's not maintained.

You need to comment out `dtoverlay=vc4-kms-v3d` in `/boot/config.txt`.
