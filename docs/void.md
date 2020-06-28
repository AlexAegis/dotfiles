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

    You **will** need a 512 MB EFI partition!

3. Run the void installer, [read the guide for more information on what each section does][guide]

    1. run `void-installer`
    
    2. Set up the bootloader   
    
        Select the disk where the EFI partition is
    
    2. Set root password
    
    3. Create user account
    
    4. Set hostname
    
    5. Set up mount points in the Partition menu, the efi partition goes to `/boot/efi`!
    
    6. Verify settings using the 'Settings' menu (Userpassword will be visible, root not)
    
    7. Run Install
    
4. Boot into the installed system!

5. Update the system

```sh
xbps-install -Su # Run this twice! First it will likely update only itself, on a second run it will update everything else!
```

[guide]: https://docs.voidlinux.org/installation/live-images/guide.html
