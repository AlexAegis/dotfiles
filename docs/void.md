# Install Void

This step-by-step guide has some slight changes compared to the online
[Installation guide][guide] and an easy to follow structure.

## Step by step Installation

This assumes US keyboard layout for the installation. Setting it to your
preference can wait until the setup is done. If you want to do the temporary
keyboard layout change anyway (for passwords it can be useful to use your
native layout) then check the [set the keyboard layout][guide] section in the
guide.

1. Create and boot into a live intallation
    1. Download [one of the live images](https://alpha.de.repo.voidlinux.org/live/current/)

        > Don't worry if they look old, it's a rolling release, everything will be fresh once installed.

    2. Create a bootable disk, the process is explained in [create-bootable-disk.md](./create-bootable-disk.md)

    3. Boot into the live installation, and log in as root. The login information is printed onto your screen. You should see a prompt.

2. Create the partitions you want to install to and use. The process is explained in [partitioning.md](./partitioning.md).

    You will need a 512 MB EFI partition!

3. Run the void installer, [read the guide for more information on what each section does][guide]

    1. run `void-installer`
    
    2. Set up the bootloader   
    
        > If this is your only installation, select a partition (512MiB) where your bootloader will be installed,
        > If you already have a bootloader, you may select `none`, and later manually add void as a bootable entry.
        > If you use already use refind, just select none, it will find it.
    
    2. Set root password
    
    3. Create user account
    
    4. Set hostname
    
    5. Set up mount points in the Partition menu
    
    6. Verify settings using the 'Settings' menu (Userpassword will be visible, root not)
    
    7. Run Install
    
    8. If you did setup the bootloader with `none`, you can set up an efistub boot entry and boot directly into Void
    
        ```
        sudo efibootmgr --disk /dev/sdX --part Y --create --label "Void" --loader /vmlinuz-linux --unicode "root=PARTUUID=$(sudo blkid /dev/sdXX) resume=PARTUUID=$(sudo blkid /dev/sdXX) rw initrd=\initramfs-linux.img" --verbose
        ```

        Where `--disk` points to the device where the efi partition is, `--part` is the number of that partition. In the `--unicode` section, root should point to the `root` partition, and `resume` to the `swap` partition. `--loader` and `initrd` might have to be renamed, check the `/boot` folder on the root partition to see these. 
        
        > If you are `root` then `sudo` is not necessary, but if you are not, don't forget to use it in the subshells too, otherwise empty results will be used.
        
        > If you added some bad entries you can delete them using `sudo efibootmgr -b 000X -B` where `000X` is the number of the boot entry. Run `efibootmgr` to list the entries.
    

[guide]: https://docs.voidlinux.org/installation/live-images/guide.html
