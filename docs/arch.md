# Install Arch

This step-by-step guide has some slight changes compared to the online
[Installation guide][guide] and an easy to follow structure. The bootloader
configuration is done beforesystem configuration so if anything would go wrong,
it's not much progress tolose. This way the hardware specific steps are
grouped together at the beginning up to step 12.

## Step by step Installation

This assumes US keyboard layout for the installation. Setting it to your
preference can wait until the setup is done. If you wan't to do the temporary
keyboard layout change anyway (for passwords it can be useful to use your
native layout) then check the [set the keyboard layout][guide] section in the
guide.

[guide]: https://wiki.archlinux.org/index.php/installation_guide

1. Create a live USB

   1. Go to <https://www.archlinux.org/download/> and download an iso with
      torrent. Always recheck the iso.
   2. Flash it, make sure to use a GPT partition table instead of MBR

2. Boot into live install

3. Make sure you have internet access in the live usb

   ```sh
   ping 8.8.8.8
   ```

4. Identify the drives you want to format

   ```sh
   fdisk -l | grep 'Disk /'
   ```

5. Open `cfdisk` to format said drive(s)

   ```sh
   cfdisk /dev/sd?
   ```

   1. Remove every partition that you don't need
   2. Create an 512 MB `EFI` boot partition if needed
      1. While on free space, select `New`
      2. For size, use `512M`. Use the `Resize` option if messed up.
      3. Change its type using the `Type` option, select `EFI System` at the top
   3. Think through the sizes of the partitions you want to make. Generally you
      will need at least 2-4G of swap partition at the end of the disk. Your
      `/` partition and optionally a `/home` partition, and anything else.
   4. Create the root partition
      1. While on free space, select `New`
      2. For size, use what you came up with. Use the `Resize` option
         if messed up.
      3. Change the type to `Linux root (x86-64)`
   5. Create any other regular partition like the previous one while leaving
      some space for the swap partition. Use the default `Linux filesystem`
      type for these.
   6. Create the swap partition
      1. While on free space, select `New`
      2. For size, accept what's left, so the disk is fully utilized.
      3. Change the `Type` to `Linux swap`
   7. Check if everything is right, you can still turn back.
   8. Select `Write` and confirm it by typing `yes`
   9. Select `Quit`

6. Format the newly created partitions

   1. Inspect your new partitions and the `device` they are assigned to

      ```sh
      lsblk -f
      ```

   2. If an `EFI` partition was created, and not already formatted
      using `FAT32`, format it.

      ```sh
      mkfs.fat -F32 /dev/sdb1
      ```

   3. If a regular partition is not already the one you want your filesystem
      to have (By default `ext4`) then format it.

      > `ext4` is a safe bet for a regular setup, it's stable and fast
      >
      > `xfs` is also fast but not as widespread as `ext4`

      ```sh
      mkfs.ext4 /dev/sdb2
      ```

   4. Don't format `swap`!

7. Turn on swap

   > `genfstab` will use this

   ```sh
   swapon /dev/sdb3
   ```

8. Mount the partitions you want the new setup to use, into the live instance

   > `genfstab` will detect every mounted partition (and swap), so you can
   > mount everything you want to be automounted by `fstab` later.
   > We will generate the `fstab` file after the `base` installation
   > so if you want to mount shared drives with data on them, mount them later
   > after the installation but before launching `genfstab`

   1. Mount the `root` partition first. `/mnt` will represent the root of the
      installed system in the perspective of the live instance

      ```sh
      mount /dev/sdb2 /mnt
      ```

   2. Mount the `boot` partition

      ```sh
      mkdir /mnt/boot
      mount /dev/sdb1 /mnt/boot
      ```

      > This will make the `boot` partition be accessible from the installed
      > system as `/boot`
      > Always use `/boot`, even with `EFI`.

   3. If you have any other partitions, mount them, even if you won't modify
      them, the `fstab` entries will be generated from this setup. Like if you
      have a separate `/home` partition.

      ```sh
      mount /dev/sd?? /mnt/home
      ```

      > This will make the `home` partition be accessible from the installed
      > system as `/home`

9. Install the system

   1. Install at least the `base`, `linux`, `linux-firmware`, `sudo` and
      `networkmanager` packages using `pacstrap`, and also install every other
      necessary packages like `git`, `vim`, `zsh`, `dash`, `man`. Or install
      them from the installed system. None of them are necessary right now
      except `networkmanager`

      > [`base`](https://www.archlinux.org/packages/core/any/base/) only
      > has a handful of utilities, like `bash`, `coreutils`, `sed`
      > and `pacman`

      ```sh
      pacstrap /mnt base linux linux-firmware networkmanager sudo
      ```

   2. If you want to edit the bootloader from the installed system, install it.

      ```sh
      pacstrap /mnt grub efibootmgr
      ```

   3. If you'd like to use the scripts available in the live install like
      `genfstab` inside the installed system, install `arch-install-scripts`

      > To learn what file, executable or script comes from which package
      > use `pacman -Fy <name>`

      ```sh
      pacstrap /mnt arch-install-scripts
      ```

   4. Install any driver that's not in `linux-firmware`. It's a good idea to
      preinstall drivers here. To let the system boot up with every needed
      driver

      - Like the proprietary nvidia driver

        ```sh
        pacstrap /mnt nvidia
        ```

      - Or the ntfs driver (Useful on dualboot systems)

        ```sh
        pacstrap /mnt ntfs-3g
        ```

   5. Install packages for microcode updates

      - Intel

        ```sh
        pacstrap /mnt intel-ucode
        ```

      - AMD

        ```sh
        pacstrap /mnt amd-ucode
        ```

   6. If you want to have extra kernels, install them too.

      > The zen linux is optimized for everyday usage and performance

      ```sh
      pacstrap /mnt linux-zen
      ```

   7. If you want to have `man` pages

      > Not necessary now

      ```sh
      pacstrap /mnt man-db man-pages texinfo
      ```

   8. Other stuff, like a text editor, a login shell

      > Not necessary now, especially if you have scripts to set up your
      > environment the only necessary packages aside from `base`, `linux` and
      > `linux-firmware` is `networkmanager` (Or an alternative)
      >
      > If you plan to do that, install `git` now

      ```sh
      pacstrap /mnt git vim zsh
      ```

10. If you want to include shared partitions like a dual-booted Windows
    system's `NTFS` partition you can do it now.

    > Make sure to have `ntfs-3g`installed in the case of mounting `NTFS`

    Create the mount point directory

    ```sh
    mkdir -p /mnt/mnt/d
    ```

    Then mount the drive up. If you don't know which one is it, use `fdisk -l`

    ```sh
    mount /dev/sda2 /mnt/mnt/d
    ```

11. Generate the `fstab` entries to have the partitions mount on startup

    ```sh
    genfstab -U /mnt > /mnt/etc/fstab
    ```

12. Step into the installed system using `chroot` to set up the bootloader

    ```sh
    arch-chroot /mnt
    ```

13. Set up the bootloader

    > Assuming `grub`

    1. Install grub

       > If not already

       ```sh
       pacman -Su grub efibootmgr
       ```

    2. Install the bootloader

       > You have to do this from `chroot` (You are here since step 12.)
       > If it says 'error: /boot doesn't look like an EFI partition.'
       > it's likely because you forgot to format it using `FAT32`
       > Run `mkfs.fat -F32 /dev/sd??` on the boot partition

       ```sh
       grub-install --target=x86_64-efi --efi-directory=/boot --bootloader-id=GRUB
       ```

    3. Auto configure grub using the images it can find in `/boot`

       > These images are the linux kernels you installed and the microcode
       > updates

       ```sh
       grub-mkconfig -o /boot/grub/grub.cfg
       ```

    > The installation is practically done, but some configurations are needed
    > and can be done while we are `chroot`ed in the new system anyway.

14. Set up timezone

    ```sh
    ln -sf /usr/share/zoneinfo/Europe/Budapest /etc/localtime
    hwclock --systohc
    ```

15. Set up locale

    1. Open up the `locale.gen` file for edit

       ```sh
       vim /etc/locale.gen
       ```

    2. Uncomment the locales you need. Also `en_US.UTF-8 UTF-8`. It should
       be on line `176`.

       ```vim
       :176<CR> " <CR> means press Return
       <DEL> " will delete the char under the cursor, which should be `#` here
       " Uncomment any other locales
       :wq
       ```

    3. Run `locale-gen`

       ```sh
       locale-gen
       ```

    4. Set your language

       ```sh
       echo 'LANG=en_US.UTF-8' > /etc/locale.conf
       ```

    5. If your keyboard layout is not `us`, set it

       > The available keymaps can be accessed through `localectl list-keymaps`

       ```sh
       echo 'KEYMAP=us' > /etc/vconsole.conf
       ```

16. Set up network

    1. Set your `hostname`

       ```sh
       echo 'myhostname' > /etc/hostname
       ```

    2. Add it to the `hosts` file

       > If you have a fixed ip address use that instead of `127.0.1.1`

       ```sh
       echo '127.0.0.1    localhost
       ::1           localhost
       127.0.1.1     myhostname.localdomain myhostname' > /etc/hosts
       ```

    3. Enable the NetworkManager

       > When using `networkmanager`

       ```sh
       systemctl enable NetworkManager.service
       ```

17. Set up root password

    ```sh
    passwd
    ```

18. Create users

    1. Create an admin user for yourself

       ```sh
       useradd -m -U -G wheel,root -s /bin/zsh alex
       ```

    2. Optionally create a guest user

       > Good for experimenting how things work in an unpriviliged environment

       ```sh
       useradd -m -U -s /bin/zsh guest
       ```

19. Reboot into the installed system

    > Keep the Live installation for emergency recoveries

20. Install your dotfiles
