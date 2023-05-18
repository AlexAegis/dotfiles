# [AppArmor](https://apparmor.net/)

> [Arch Wiki](https://wiki.archlinux.org/title/AppArmor)

Linux application security system

## Boot

AppArmor has to be added as a kernel paramter for it to work. This is already
done in my [Refind boot stanza](../refind/resources/boot/EFI/refind/config/boot-stanzas.conf)
and as such this module is a dependency of the refind module too.
