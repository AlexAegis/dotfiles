# [NTFS](https://en.wikipedia.org/wiki/NTFS)

> [Arch Wiki](https://wiki.archlinux.org/index.php/NTFS-3G)

Windows filesystem

## Shared drives

On windows, linux filesystem drivers are hard to get by and tend to be buggy.
On linux however the `ntfs-3g`, despite being a FUSE (Filesystem in userspace)
driver is a really good and mature solution.

For shared drives between Windows and Linux, the best choice is NTFS.

### Important

**Fast startup must be disabled for Windows!** The reason being that fast
startup does a mixed shutdown/hibernation. A hibernated drive can't be
modified! It can be forced if you tell ntfs-3g to delete the hibernation file
but it will lead to lost and potentially corrupted data! Disabling fast
startup solves this by not doing this kind of hibernation and then the drive
can always be mounted in read-write mode.

> If you boot to Windows from an SSD, the performance hit should be very
> minimal. I'm using an NVME SSD and I feel no difference. But try it, and see
> it yourself.

## Usage

Once installed, just mount up the NTFS partition the regenerate the `fstab`
entry in `/etc`.

```sh
genfstab -U / >> /etc/fstab
```

> Note: If you are on a live installation medium and you want to generate this
> for the host system then the root will be different here!
