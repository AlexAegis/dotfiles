# [rEFInd](https://www.rodsbooks.com/refind/)

> [Arch Wiki](https://wiki.archlinux.org/index.php/REFInd)

Since installing a bootloader is quite involved this module does not install
it for you, but it does configure one if it finds one.

- `/mnt/refind/EFI/refind`

  if you have refind on another efi partition, mount it to
  `/mnt/refind` so it can be used as the `--root` argument

- `/boot/efi/EFI/refind`

  this is it's default location if it's on the same efi partition as the host os

## Installation

Using `refind-install` by default will create a refind folder in `/boot/efi`
or closest equivalent. If you want to install on a specific drive, mount that
partition

## Autodetecting OS types

If the parent folder of the discovered efi file is recognizable then you will
get OS specific icons automatically. These names are case-insensitive.

`/EFI/<OS>/grubx64.efi`

Examples:

- arch
- void
- freebsd
- manjaro
