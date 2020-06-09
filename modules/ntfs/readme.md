# [NTFS](https://en.wikipedia.org/wiki/NTFS)

> [Arch Wiki](https://wiki.archlinux.org/index.php/NTFS-3G)

Windows filesystem

## Shared drives

On windows, linux filesystem drivers are hard to get by and tend to be buggy.
On linux however the `ntfs-3g`, despite being a FUSE (Filesystem in userspace)
driver is a really good and mature solution.

For shared drives between Windows and Linux, the best choice is NTFS.

## Usage

Mount up the NTFS partition the regenerate the `fstab` entry in `/etc`.

```sh
genfstab -U / >> /etc/fstab
```

> Note: If you are on a live installation medium and you want to generate this
> for the host system then the root will be different here!
