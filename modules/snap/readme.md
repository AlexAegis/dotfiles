# [Snap](https://snapcraft.io/)

> [Arch Wiki](https://wiki.archlinux.org/index.php/Snap)

## AppArmor

The [apparmor](../apparmor/readme.md) module installs it and the
[refind](../refind/readme.md) boot module enables it.

To check its status run `sudo apparmor_status`

## Updates

Snaps automatically update (usually) every 4 hours.

To check how often they do update:

```sh
snap refresh --time
```

To do a manualy update:

```sh
snap refresh
```
