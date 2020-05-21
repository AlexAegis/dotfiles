# [wal](https://github.com/dylanaraps/pywal)

This module can set the wallpaper, and also generates a colorcheme based on it
which will the be applied on all terminals and programs that support it.

Note that if a wallpaper doesn't have enough colors in it, some backends
will simply crash. The default `wal` backend can deal with this, but the
preferred `haishoku` or `colorz` does not.

## [Supported Modules](https://github.com/dylanaraps/pywal/wiki/Customization)

### [polybar](../polybar)

Uses `accent = ${xrdb:color4:#222}` as the accent color.

### [Vim](../vim)

not yet

### [VS Code](../vscode)

Install the dlasagno.wal-theme extension so vs code can pick up wals color
scheme
