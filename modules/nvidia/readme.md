# [nvidia](https://www.nvidia.com/en-us/)

> [Arch Wiki](https://wiki.archlinux.org/index.php/NVIDIA)

## On arch

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
