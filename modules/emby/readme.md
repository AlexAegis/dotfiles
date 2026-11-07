# [Emby](https://emby.media/index.html)

> [Arch Wiki](https://wiki.archlinux.org/title/Emby)

## Updating

The module installs the newest stable release, resolved from the GitHub
releases api, so `pontup` keeps it current. Prereleases are never picked up.
When the api is unreachable the `emby_fallback_version` in `0.root.pacman.sh`
is used instead, so bump that when it drifts too far behind.
To see the available releases visit
<https://github.com/MediaBrowser/Emby.Releases/releases>

## Moving the Emby database

The installed `link_emby_database.sh` script links `/var/lib/emby` to the
folder that its argument names. `1.root.sh` runs it with `/var/media/emby`.

- `/var/lib/emby` is a link: only the link changes, the data stays in the old
  folder
- new folder does not exist: the data moves there
- new folder exists: the data moves into it as `prev-server-backup-<date>`.
  Emby then uses the files already in the new folder.
