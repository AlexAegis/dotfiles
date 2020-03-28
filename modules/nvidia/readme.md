# nvidia

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
