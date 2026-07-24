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

The installed `link_empy_database.sh` script can move and link the emby data
directory anywhere. By default it does it to `/var/media/emby`
