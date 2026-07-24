# Arch

Arch Linux specific system setup

## Mirrors

Using `reflector` you can automatically rank mirrors by download rate.

```sh
reflector --latest 20 --protocol https --sort rate
```

Or automatically update your mirrorlist using:

```sh
sudo reflector --latest 20 --protocol https --sort rate --save /etc/pacman.d/mirrorlist
```

## System package unique features

> these are meant to be replicated on other systems or find a common way

* <https://wiki.archlinux.org/index.php/Activating_numlock_on_bootup>

  > BUG: Currently only the light turns on and not the functionality

## Handling `.deb` packages

Install `debtap` from the AUR

To repackage a `.deb` file to `.pkg` format:

```sh
sudo debtap -u
debtap package_name.deb
sudo pacman -U package_name.pkg
```
