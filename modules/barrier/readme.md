# [Barrier](https://github.com/debauchee/barrier)

Share peripherials between systems

> [Arch Wiki](https://wiki.archlinux.org/title/Barrier)

## Server

On the machine where you host your peripherials, launch barrier as the server.

The server has to be configured with a screen matching the hostname of the
client for it accept that client.

### Windows DPI Fix

> https://github.com/debauchee/barrier/issues/1638

1. Go to `C:\Program Files\Barrier`
2. Right click `barriers.exe`, Properties
3. Go to Compatibility Tab
4. Click `Change settings for all users`
5. Click `Change high DPI settings`
6. Tick in `Use this setting to fix...`
7. Tick in `Override high DPI scaling behavior`
8. OK. OK. OK.
9. Restart server (Completely quit it from the taskbar)

### SSL

By default an SSL cert is missing for barrier to use so generate one:

> From: https://github.com/debauchee/barrier/issues/231#issuecomment-958800595
> This module already does this on the client but you also need to run it on
> the server

```powershell
cd ~\AppData\Local\Barrier\SSL
openssl req -x509 -nodes -days 365 -subj /CN=Barrier -newkey rsa:4096 \
    -keyout Barrier.pem -out Barrier.pem
```

## Client(s)

> I couldn't get a system level barrier client working, but user level is fine.
> This means you can't use Barrier to operate a login manager.

### Crypto module

Add the fingerprint(s) of the server(s) you trust to
`$XDG_DATA_HOME/barrier/SSL/Fingerprints/TrustedServers.txt`.
