# [samba](https://www.samba.org/)

> [Arch Wiki](https://wiki.archlinux.org/title/samba)

The standard Windows interoperability suite

## Mapped Network Drives

Available at `\\HOSTNAME.local\SHARENAME`

## Reload

```sh
sudo systemctl reload smb.service nmb.service
```
