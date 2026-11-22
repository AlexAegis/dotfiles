# [FreeBSD](https://www.freebsd.org/)

## Before pont

pont runs root steps with sudo and links files with stow. FreeBSD base has
neither. The `sudo` module does not apply here. The `stow` module has no `pkg`
step. Set up both by hand before the first pont run. Run these steps as root in
the dotfiles repository. Replace `<user>` with your user name:

1. Install sudo and stow: `pkg install sudo stow`.
2. Add the user to `wheel`: `pw groupmod wheel -m <user>`.
3. Check the sudoers file of the `sudo` module:
   `visudo -c -f modules/sudo/resources/10-wheel`.
4. Install it to the drop-in folder that sudo from pkg reads:
   `install -o root -g wheel -m 440 modules/sudo/resources/10-wheel /usr/local/etc/sudoers.d/`.
5. Log in again as the user, so that the new group applies.

## Services

Enable services in `/etc/rc.conf`, by hand or with `sysrc`.

The root step enables sshd. When no `ifconfig_` variable is set, it also sets
DHCP for the first network adapter that is up, and fails when none is up. Both
settings apply at the next boot.
