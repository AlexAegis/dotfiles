# Install Arch

Check the [Installation guide](https://wiki.archlinux.org/index.php/installation_guide) to see if anything changed

1. Boot into live install
2. Follow the install guide

Notes about the guide:

- For partitioning use `cfdisk`, it's a curses based graphical frontend for fdisk
- Create a separate /home partition, for root, 25 GB is enough
- Format the efi partition with FAT32, don't format the swap and the rest with ext4
- also pacstrap `grub`, `networkmanager`, `vim`, `man-db`, `man-pages`, `zsh` and `intel-ucode` on intel systems or `amd-ucode` for amd systems.

    ```
    pacstrap /mnt base linux linux-framework grub networkmanager vim zsh man-db man-pages
    ```

3. [Install GRUB](https://wiki.archlinux.org/index.php/GRUB#Installation_2) (Or other bootloader)

> Run these from the installed system (`chroot`)

```zsh
grub-install --target=x86_64-efi --efi-directory=/boot --bootloader-id=GRUB
```

Configure microcode update on GRUB (If a ucode image is installed)

```zsh
grub-mkconfig -o /boot/grub/grub.cfg
```

4. Enable networking

If you installed NetworkManager you'll also have to enable it's systemd unit by

```zsh
systemctl enable NetworkManager.service
```

> `enable` allows it to run it on boot, but not `start` it right away.
