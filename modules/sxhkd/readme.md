# [sxhkd](https://github.com/baskerville/sxhkd)

> [Arch Wiki](https://wiki.archlinux.org/index.php/Sxhkd)

This module sets up `sxhkd` as a systemd user daemon and loads every config
from `~/.config/sxhkd/sxhkdrc.d/*`

## Auto config reload

Systemd paths are running together with the service it spawns so it can't
trigger more than once. Also, when watching folders they can't follow the
symlinks inside that folder because it only watches the `inodes`. It would
follow when pointing to a single symlink.

That's why I switched to `entr`, and while it still works the same way,
you can supply multiple files to it (following each if it's a symlink),
and with the `-d` flag it can also watch the parent directory and exit if that
changes. Combining this with an infinite loop we can watch for new files too.
The only thing the `-d` flag doesn't watch is file removal.
