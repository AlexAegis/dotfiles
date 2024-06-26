# Install Arch

This step-by-step guide has some slight changes compared to the online
[Installation guide][guide] and an easy to follow structure. The bootloader
configuration is done before system configurations so if anything would go
wrong, it's not much progress to lose. This way the hardware specific steps are
grouped together at the beginning, up to step 12.

## Step by step Installation

This assumes US keyboard layout for the installation. Setting it to your
preference can wait until the setup is done. If you want to do the temporary
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

5. If the drive is an nvme drive, you may want to flush it to restore factory
   write speeds

  > [Arch Wiki SSD Memory Cell Clearing](https://wiki.archlinux.org/title/Solid_state_drive/Memory_cell_clearing#Format_command)

  Install `nvme-cli`

  ```sh
  pacman -Syu nvme-cli
  ```

  Then format the nvme drive, if it fails with `-s 2` because it's not supported
  try with `-s 1`, older drives generally do not support `-s 2`.

  > This will wipe the drive completely

  ```sh
  nvme format /dev/nvme0n1 -s 2 -n 1
  ```

6. Open `cfdisk` to format said drive(s)

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

7. Format the newly created partitions

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
      > Seriously, for a simple 1 disk setup `ext4` is a really solid choice.
      > Unless you need `btrfs` features just use `ext4`

      ```sh
      mkfs.ext4 /dev/sdb2
      ```

   4. Make swap!

      ```sh
      mkswap /dev/sdb3
      ```

8. Turn on swap

   > `genfstab` will use this

   ```sh
   swapon /dev/sdb3
   ```

9. Mount the partitions you want the new setup to use, into the live instance

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

10. Install the system

   1. Install at least the `base`, `base-devel`, `linux`, `linux-firmware`,
      `sudo`, `git` and `networkmanager` packages using `pacstrap`.
      Among these only `base`, `linux`, `linux-firmware` and `networkmanager`
      packages are the most important to install because after that you`ll have
      pacman and network access to install anything else from the installed
      system. But having these others install too can make things easier.

      > `pont` users only have to install `git` manually on top of the
      > mandatory ones.
      >
      > [`base`](https://www.archlinux.org/packages/core/any/base/) only
      > has a handful of utilities, like `bash`, `coreutils`, `sed`
      > and `pacman`
      >
      > `base-devel` is needed to install AUR packages, can be installed later

      ```sh
      pacstrap /mnt base base-devel linux linux-firmware networkmanager sudo
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
      >
      > ! But some things might break under different kernels
      > Always switch back to the original kernel if some hardware related
      > thing failes

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

11. If you want to include shared partitions like a dual-booted Windows
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

12. Generate the `fstab` entries to have the partitions mount on startup

    ```sh
    genfstab -U /mnt > /mnt/etc/fstab
    ```

13. Step into the installed system using `chroot` to set up the bootloader

    ```sh
    arch-chroot /mnt
    ```

14. Set up the bootloader (Either Refind or Grub)

  1. refind

    1. Install Refind

      ```sh
       pacman -Su refind
       refind-install
      ```
    
    2. If you already have a refind config (from a previous installation)
       copy it to `/boot` and **update all UUID's** in the 
       `/boot/refind_linux.conf` file and for any boot-stanza you have under
       `/boot/EFI/refind/` (I keep all of them in
       `/boot/EFI/refind/config/boot-stanzas.conf`)

       > The `fstab` file you just generated contains the relevant UUID's!
    
    3. If you copied refind, make sure to run `refind-install` again so it's
       up-to-date

      
  2. grub

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

15. Add kernel modules

    1. nvidia

        Edit `/etc/mkinitcpio.conf`

        ```sh
        vim /etc/mkinitcpio.conf
        ```

        And add the following modules to the MODULES list.

        ```sh
        MODULES=(nvidia nvidia_modeset nvidia_uvm nvidia_drm)
        ```

    2. Finally regenerate the initramfs

      ```sh
      mkinitcpio -P
      ```

16. Set up timezone

    ```sh
    ln -sf /usr/share/zoneinfo/Europe/Budapest /etc/localtime
    hwclock --systohc
    ```

17. Set up locale

    1. Open up the `locale.gen` file for edit

       ```sh
       vi /etc/locale.gen
       ```

    2. Uncomment the locales you need. Also `en_US.UTF-8 UTF-8`.

       ```vim
       /#en_US.UTF<CR> " <CR> means press Return
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

18. Set up network

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

19. Set up root password

    ```sh
    passwd
    ```

20. Create users

    1. Create an admin user for yourself

       ```sh
       useradd -m -U -G wheel,root -s /bin/zsh <user>
       ```

    2. Change your password

       ```sh
       passwd <user>
       ```

    3. Give passwordless sudo access to the wheel group

       Launch `visudo`, and uncomment or add this line:

       ```ini
       %wheel ALL=(ALL:ALL) NOPASSWD: ALL
       ```

       If you only want to add it to your user alone:

       > The `visudo` is not necessary here, it only does a syntax check \
       > If you are confident enough, leave it out.

       ```sh
       echo '<user> ALL=(ALL:ALL) NOPASSWD: ALL' > /etc/sudoers.d/05-<user>
       chmod 440 /etc/sudoers.d/05-<user>
       ```

       And optionally do a syntax check with `visudo` if you didn't edit the
       file with that:

       ```sh
       visudo -cf /etc/sudoers.d/05-<user>
       ```

       > It starts with `05` so that most of the later installed sudoer files
       > can overwrite it. For example, a `10-wheel` sudoer file that will give
       > even more permissions (NOPASSWD: ALL) will only have an effect on this
       > user if it's loaded after this, less permissive one.

    4. Optionally create a guest user and delete unused users

       > Good for experimenting how things work in an unpriviliged environment

       ```sh
       useradd -m -U -s /bin/zsh guest
       ```

       ARM images come with a default `alarm` user, you can delete that:

       ```sh
       userdel alarm
       ```

21. Reboot into the installed system

    > Keep the Live installation for emergency recoveries

22. Install your dotfiles.

    > If you don't have one, make one. Saves a lot of time.
