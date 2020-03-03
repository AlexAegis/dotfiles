# x

TODO: This whole mouse config does not work, right now I'm using imwheel. might be a better option as it can be user specific.

## Mouse config

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

TODO:
<https://medium.com/njiuko/using-fzf-instead-of-dmenu-2780d184753f>
