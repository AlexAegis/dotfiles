# [X](https://wiki.archlinux.org/index.php/Xorg)

TODO: Implement config reload on display change using xrandr

## Inspection

With `xprop`, clicking on a window lets you inspect it's various X properties
like it's wm_class, that can be used to identify elements in compositor
configurations.

```sh
xprop
```

## [Monitor Layout](https://wiki.gentoo.org/wiki/Xorg/Multiple_monitors)

### Config based

To use a different layout than the default create it a file for it:
To use a `.conf` file create a `50-layout-<layout-name>.conf` file
in the `~/.config/xorg/layout/` folder or here in this module in the same
place.

> To use this set the `LAYOUT_USE_CONF` variable to `1` (default)

#### Config Generation

To generate an `xorg.conf` file, when using nvidia drivers you can use
`nvidia-settings` to generate a config file

### Script based

If you don't have the rights to change the config you can fall back to a
script based layout change using `xrandr`. You have to make a layout script
in the same place as the installable conf files in the previous section, but
named `<layout-name>.layout`.
This will set the layout after logging in, causing some delay after logging in.

> To use this set the `LAYOUT_USE_CONF` variable to `0`

#### Script Generation

To generate an `xrandr` layout with a gui, use `arandr`

## Notes

### [Writing user services that depends on x](https://superuser.com/questions/759759/writing-a-service-that-depends-on-xorg)

This module provides an `xsession.target` to bind services to.
This target is then triggered by the `~/.xsession` file

## Mouse config

TODO: This whole mouse config does not work, right now I'm using imwheel.
might be a better option as it can be user specific.

Using `lsusb` check the id of your pointer device.

Example:

```sh
Bus 001 Device 003: ID 0b05:181a ASUSTek Computer, Inc. ROG GLADIUS
```

It returns the id in `<vid>:<pid>` format.

In the `71-mouse-local.hwdb` file it has to be present in a `mouse:usb:v<vid>p<pid>:name:*:` format

To apply the settings

```sh
udevadm hwdb --update
udevadm trigger /dev/input/event${id}
```

Where id is the id of your mouse according to `xinput`.

```sh
xinput list
```

To get the device id

```sh
xinput list-props 15
```

To get the property ids, look for Device Node when using libinput

So for example:

```sh
sudo udevadm hwdb --update
sudo udevadm trigger /dev/input/event4
```
