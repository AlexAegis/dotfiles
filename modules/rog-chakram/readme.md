# ASUS Rog Chakram

The joystick competes with the xbox controller so it's better to just disable it:

> `/usr/lib/udev/rules.d/99-chakram.rules`

```ini
SUBSYSTEM=="input", ATTRS{name}=="ASUSTeK ROG CHAKRAM", ATTRS{phys}=="*/input0", SYMLINK+="input/event_0"
SUBSYSTEM=="input", ATTRS{name}=="ASUSTeK ROG CHAKRAM", ATTRS{phys}=="*/input1", SYMLINK+="input/event_0"
SUBSYSTEM=="input", ATTRS{name}=="ASUSTeK ROG CHAKRAM", ATTRS{phys}=="*/input2", SYMLINK+="input/event_0"
SUBSYSTEM=="input", ATTRS{name}=="ASUSTeK ROG CHAKRAM", ATTRS{phys}=="*/input3", RUN+="/usr/bin/rm /dev/input/%k"
```

Then reload

```sh
sudo udevadm control --reload
```
