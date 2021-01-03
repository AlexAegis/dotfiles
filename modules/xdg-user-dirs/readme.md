# [xdg-user-dirs](https://wiki.archlinux.org/index.php/XDG_user_directories)

> [Arch Wiki](https://wiki.archlinux.org/index.php/XDG_user_directories)

[Freedesktop Basedir Specification](https://specifications.freedesktop.org/basedir-spec/latest/)

This module contains the XDG variable definitions for User directories

The rest of the modules should follow this, and set this module as their
dependency if they wish to utilize it.

## Notes

The `user-dirs.dirs` file was made available to the general environment
by creating a symlink in the `environtment.d` folder

```sh
ln -s ../user-dirs.dirs 11-xdg-user-dirs.conf
```

TODO: Systemd runtime cleanup on logout shutdown and restart
