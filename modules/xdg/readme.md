# [xdg](https://wiki.archlinux.org/index.php/XDG_Base_Directory)

[Freedesktop Basedir Specification](https://specifications.freedesktop.org/basedir-spec/latest/)

This module contains the XDG variable definitions

The rest of the modules should follow this, and set this module as their
dependency if they wish to utilize it.

## Scructural Dependencies

In the `home` module, the `.profile` file depends on the internal settings
of this module because it has to reference the main XDG variables in order
to load them.

```sh
. "$HOME/.config/environment.d/10-xdg.conf"
```

If this module is changed in this aspect, you have to manually change that.

## Notes

See also [xdg-user](../xdg-user) for configurations of folders like `music`,
`documents` and such. This module is intended to provide the bare minimum of
XDG configuration as a lot of other modules depend on it.
