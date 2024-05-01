# [nvidia](https://www.nvidia.com/en-us/)

> [Arch Wiki](https://wiki.archlinux.org/index.php/NVIDIA)
>
> [Drivers](https://www.nvidia.com/en-us/geforce/drivers/)

## On Arch

```sh
sudo vim /usr/lib/modprobe.d/nvidia.conf
```

looks like this:

```conf
blacklist nouveau
options nvidia NVreg_RegisterForACPIEvents=1 NVreg_EnableMSI=1
```

## Config generation

Based on current hardware

```sh
nvidia-xconfig
```

## OpenGL Environment

<http://us.download.nvidia.com/XFree86/FreeBSD-x86/319.32/README/openglenvvariables.html>

## Early KMS

TODO: Automatize this part

```sh
sudo vim /etc/mkinitcpio.conf
```

Add `nvidia` to the `MODULES` array

then regenerate the `initramfs`

```sh
sudo mkinitcpio -p linux
```
