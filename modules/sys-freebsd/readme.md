# [FreeBSD](https://www.freebsd.org/)

## Services

Enable services in `/etc/rc.conf`, by hand or with `sysrc`.

The root step enables sshd. When no `ifconfig_` variable is set, it also sets
DHCP for the first network adapter that is up, and fails when none is up. Both
settings apply at the next boot.
