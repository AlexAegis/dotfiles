# Arch

Arch Linux specific system setup

## Mirrors

Using `rate-mirrors` you can automatically evaluate mirrors based on their speed.

```sh
rate-mirrors --protocol https arch 
```

Or automatically update your mirrorlist using:

```sh
rate-mirrors --allow-root --protocol https arch | grep -v '^#' | sudo tee /etc/pacman.d/mirrorlist
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
