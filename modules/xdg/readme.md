# [xdg](https://wiki.archlinux.org/index.php/XDG_Base_Directory)

[Freedesktop Basedir Specification](https://specifications.freedesktop.org/basedir-spec/latest/)

This module contains the XDG variable definitions

The rest of the modules should follow this, and set this module as their
dependency if they wish to utilize it.

## Notes

The `user-dirs.dirs` file was made available to the general environment
by creating a symlink in the `environtment.d` folder

```sh
ln -s ../user-dirs.dirs xdg-user-dirs.env
```

TODO: Systemd runtime cleanup on logout shutdown and restart
