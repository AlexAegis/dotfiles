# [btrfs](https://btrfs.wiki.kernel.org/)

> [Arch Wiki](https://wiki.archlinux.org/title/btrfs)

## Configuration

### `/etc/fstab` configuration

1. Set `type` to `btrfs`
2. Set `fs_passno` (last field) to `0`
3. Enable compression and auto defrag by adding
   `compress-force=zstd,autodefrag` to the options.

## Maintanence

### Check used space

> This module also aliases `df` to this if btrfs is available

```sh
btrfs filesystem df
```

### Mounting

> Use these options for `/etc/fstab` configurations too

```sh
mount -o compress-force=zstd,autodefrag /dev/<device> /<target>
```

### [Rebalance](https://wiki.archlinux.org/title/btrfs#Balance)

> This takes a while!

```sh
btrfs balance /
```

### [Recompress](https://wiki.archlinux.org/title/btrfs#Compression)

> This also takes a while, it rewrites every single file on
> the drive to apply compression

```sh
btrfs filesystem defragment -r -v -czstd /

```

### [Defragment](https://wiki.archlinux.org/title/btrfs#Defragmentation)

```sh
btrfs filesystem defragment -r /
```

### [Converting ext4 to btrfs](https://wiki.archlinux.org/title/btrfs#Ext3/4_to_Btrfs_conversion)

Boot into a different system and mount the to-be converted device with the
same mount structure as it's used on the device.

1. Create a backup to be sure

   > <https://wiki.archlinux.org/title/Full_system_backup_with_tar>

   ```sh
   mkdir -P /mnt/root
   mount <device> /mnt/root
   # Other mounts
   chroot /mnt/root
   tar --acls --xattrs -cpvf backup-<date>-<other-identifiers>.tar.gz /
   exit
   mv /mnt/root/backup-<date>-<other-identifiers>.tar.gz <somewhere-else>
   unmount -R /mnt/root
   ```

2. Convert

   ```sh
   btrfs-convert /dev/<partition>
   ```

3. Update `/etc/fstab` and boot settings

   > UUID's of the partitions are new!

   1. Set `type` to `btrfs`
   2. Set `fs_passno` (last field) to `0`
   3. Enable compression and auto defrag by adding
      `compress-force=zstd,autodefrag` to the options.

4. Verify that it boots up and works
5. Remove backup subvolume

   ```sh
   btrfs subvolume delete /ext2_saved
   ```

6. [Rebalance](https://wiki.archlinux.org/title/btrfs#Balance)

   > This takes a while!

   ```sh
   btrfs balance start --bg --full-balance /
   ```

7. [Recompress](https://wiki.archlinux.org/title/btrfs#Compression)

   > This also takes a while, it rewrites every single file on
   > the drive to apply compression

   ```sh
   btrfs filesystem defragment -r -v -czstd /
   # and any other mountpounts (?)
   ```

8. [Defragment](https://wiki.archlinux.org/title/btrfs#Defragmentation)

   ```sh
   btrfs filesystem defragment -r /
   ```
