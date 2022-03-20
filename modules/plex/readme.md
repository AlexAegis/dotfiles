# [Plex](https://www.plex.tv/)

> [Arch Wiki](https://wiki.archlinux.org/index.php/plex)

Media server

Web interface available under `host:32400/web`

## Disabling Auth on local network

Add `192.168.0.1/255.255.255.0` (Or whatever your local network is in
gateway/mask format) To your servers Network settings, to the
`List of IP addresses and networks that are allowed without auth` field.

Connecting to Plex from this network then wont require a pin.

## Disabling unwanted services

Go to `web/index.html#!/settings/online-media-sources`

And disable everything you don't need. You might also have to unpin these.

## Removing old servers

Go to `web/index.html#!/settings/devices/all`

Remove unwanted servers.
