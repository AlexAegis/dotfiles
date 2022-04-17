# Pi Network Server

A module intended to configure an Pi server running Arch ARM. An XFCE desktop
and LightDM with autologin. The graphics environment does not load
automatically only when launched through ssh

## Custom

## Network

### Routing

The associated modules assume the server is running on the fixed ip
`192.168.0.222`. This IP is aliased behind `pi` where it's possible (For
example in the `ssh` config of the [`pi-network-client`](../pi-network-client)
module).

> Check your local networks setting to set up a fixed ip agains the MAC
> address of the pi server.

## Pi

### Bootconfig

You can add kernel flags in `/boot/cmdline.txt`

Add `root=/dev/sdXX` to point to your root partition.

And add `quiet` to disable console logging.

### Video

Use a proper video driver to avoid software rendering like
[`xf86-video-fbturbo-git`](https://archlinuxarm.org/packages/armv7h/xf86-video-fbturbo-git)

To enable 4K@60 add `hdmi_enable_4kp60=1` to `/boot/config.txt`.

### Overscan

Edit `/boot/config.txt` and add `disable_overscan=1` to disable black bars.

Alternatively they can also be customized:

```ini
overscan_left=10
overscan_right=10
overscan_top=15
overscan_bottom=15
```

### [Overclocking](https://qengineering.eu/overclocking-the-raspberry-pi-4.html)

Edit the `/boot/config.txt` and add the following:

> Only overclock if you have a proper heatsink attached, do not exceed 80 C°

```ini
over_voltage=6
arm_freq=2000
gpu_freq=600
```

For a safer overclock:

```ini
over_voltage=3
arm_freq=1800
gpu_freq=600
```

The default are for a Raspberry Pi 4:

```ini
over_voltage=5
arm_freq=1500
gpu_freq=500
```

Check `lscpu` to verify clockspeed.
