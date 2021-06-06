# [grub](https://www.gnu.org/software/grub/)

> [Arch Wiki](https://wiki.archlinux.org/index.php/GRUB)

This module can only be installed with a running X server. "But that's
ridicolous!" you might think, and rightlz so, but keep in mind that the point
of this module is only some extra grub configuration, by the time you can
use this module, you have a working bootloader.

The running X server is needed to get the display resolution and configure
grub's resolution (and through that, the tty's resolution).

I could, in theory get the same information from the hardware's EDID but it's
hard to work with.
