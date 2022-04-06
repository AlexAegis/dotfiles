# Setting up a Raspberry Pi 4 v1.4 with 64-bit Arch Linux

> As of 2022-04-06, the most recent RPi4 8gb model and the most recent 64-bit
> arch image doesn't work together so a few manual adjustments need to be made

You have a miriad of ways of setting this thing up, but you'll need to boot
onto something first. You can choose to continue using an SD-Card...

> When using an sd-card, get a faster, modern card. The speed difference can be
> huge. Consult with the [wiki](https://en.wikipedia.org/wiki/SD_card) to see
> the differences. Also get at least a 64GB variant to leave some room to breathe.
> The extra space can also be used to split `/home` off onto a separate partition.
> Then the entire system could run off the SD-Card and data stored on an external
> USB device

...or a USB device like an SSD. For that to work you need an EEPROM newer
than 2020 sept 3. if older you can boot up Raspbian and use the
[`Raspberry Imager`](https://www.raspberrypi.com/software/) to update it.
You can also create a bootable Raspian SD-Card using it from any other system.

> On some linux systems (Raspbian, Arch) you can just install the `rpi-imager`
> package.

## Create a bootable SD Card or USB device

Just use the
[script](../../modules/pi-network-server/resources/setup-arch-aarch64-device.sh)
I wrote. It handles sd cards and usb devices, and can set your device up with
2 or 3 partitions complete with fstab config. If you wish to do it manually or
you're just interested what fixes you need to make in addition to the
[arch install guide](https://archlinuxarm.org/platforms/armv8/broadcom/raspberry-pi-4)
(which is not up to date, you don't need to modify your fstab like that, it is
still mmcblk0 and not 1. But the main problem is that the tarball at
<http://os.archlinuxarm.org/os/ArchLinuxARM-rpi-aarch64-latest.tar.gz> is
really old and the kernel inside it just doesn't work. The SD Card and USB
keyboard is not working and it wont find devices by neither UUID nor PARTUUID
and the default boot.txt tries to use that. And it also has 2 lines
misconfigured. The script I linked "patches" these up but you still need to SSH
in and do a system upgrade (and keyring initialization) to bring everything
together. Then you'll be ready to use it like you would with the `armv7` which
works out of the box.

Thank you so much kyoshiro for figuring this out!
<https://archlinuxarm.org/forum/viewtopic.php?f=67&t=15422&start=20#p68473>
