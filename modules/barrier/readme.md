# [Barrier](https://github.com/debauchee/barrier)

Share peripherials between systems

> [Arch Wiki](https://wiki.archlinux.org/title/Barrier)

## Server

On the machine where you host your peripherials, launch barrier as the server.

The server has to be configured with a screen matching the hostname of the
client for it accept that client.

## Client(s)

> I couldn't get a system level barrier client working, but user level is fine.
> This means you can't use Barrier to operate a login manager.

### Crypto module

Add the fingerprint(s) of the server(s) you trust to
`$XDG_DATA_HOME/barrier/SSL/Fingerprints/TrustedServers.txt`.
