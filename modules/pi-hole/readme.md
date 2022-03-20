# [Pi-Hole](https://pi-hole.net/)

> [Arch Wiki](https://wiki.archlinux.org/title/Pi-hole)

DNS Sinkhole

## [Pihole updatelist](https://github.com/jacklul/pihole-updatelists)

A service that would update curated blocklists, and keep them updated.

### Running it manually

Using the systemd service, just start it:

```sh
sudo systemctl start pihole-updatelists.service
```

To check it's status:

```sh
sudo systemctl status pihole-updatelists.service
```
