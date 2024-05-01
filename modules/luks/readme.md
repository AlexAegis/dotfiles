# [LUKS](https://wiki.archlinux.org/title/Dm-crypt/Encrypting_an_entire_system)

Block Device encyption

> [Arch Wiki](https://wiki.archlinux.org/title/Dm-crypt/Encrypting_an_entire_system)

## Creating an encrpyted device

> Always double-check the name of your device using `lsblk`!

First format and partition a drive like you would otherwise, but you don't
immediately make a filesystem on it.

```sh
cfdisk /dev/nvme0n1
```

Then you can format using LUKS

```sh
cryptsetup -y -v luksFormat /dev/nvme0n1p2
```

## Mounting an encrypted device

> In case you get an error trying to `cryptsetup open` your new device
> try rebooting, it can happen if the device was mounted during this session

```sh
cryptsetup open /dev/nvme0n1p2 rocket
mkfs.ext4 /dev/mapper/rocket
mount /dev/mapper/rocket /mnt/rocket
```
