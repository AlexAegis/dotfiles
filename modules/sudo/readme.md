# [sudo](https://www.sudo.ws/)

> [Arch Wiki](https://wiki.archlinux.org/index.php/sudo)\
> [Gentoo Wiki](https://wiki.gentoo.org/wiki/Sudo)

This module installs custom sudoers config in `/etc/sudoers.d/`.

The install script checks the syntax of `./sudoers.custom` with `visudo`
and when installing the script it changes to owner and group to root and
the mode to `440` which is the same as for the `/etc/sudoers` file.

## Note

`sudo` will read each file in `/etc/sudoers.d`, skipping file names that
end in `~` or contain a `.` character to avoid causing problems with
package managers or the editors temporary/backup files.
