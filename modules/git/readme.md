# [git](https://git-scm.com/)

> [Arch Wiki](https://wiki.archlinux.org/index.php/git)

Version control system

## Tips

### Config provenance

To see which config comes from where

```sh
git config --list --show-origin
```

or if this module is installed just use the git alias of:

```sh
git lc
```

> **l**ist **c**onfig

### Safe force pushing

Let's you overwrite the commits on remote, but only if what you're overwriting
on the remote is the same as what you overwrote locally (meaning, if someone
else pushed in the meanwhile you won't destroy their work)

```sh
git push --force-with-lease
```

### Maintenance

Marks a repository, so that git in the background will do maintenance tasks
periodically.

```sh
git maintenance start
```

Example output when run for the first time

```sh
Created symlink ~/.config/systemd/user/timers.target.wants/git-maintenance@hourly.timer → ~/.config/systemd/user/git-maintenance@hourly.timer.
```

## Notes

Useful talk from Scott Chacon: https://www.youtube.com/watch?v=aolI_Rz0ZqY

TODO: <https://github.com/github/hub>
