# [Systemd](https://github.com/systemd/systemd)

> [Arch Wiki](https://wiki.archlinux.org/index.php/Systemd)

## [User](https://wiki.archlinux.org/index.php/Systemd/User)

TODO:
Test:

```sh
sudo systemd-analyze verify every service
```

## Auto daemon reload

Whenever a service file changes in `~/.config/systemd/user/`, watched by
`user-services.path`, `daemon-reload.service` kicks in an reloads the daemon.
The only exception being `daemon-reload.service`, because when that changes
systemd refuses to run it until the daemon has been reloaded.

To manually reload the user daemon:

```sh
systemctl --user daemon-reload
```

## Expectations

If any user service changes, reload the daemon.

If any file that a service depends on changes, reload the service.
