# [Ruby](https://www.ruby-lang.org/en/)

> [Arch Wiki](https://wiki.archlinux.org/index.php/Ruby)

## [RVM](https://wiki.archlinux.org/index.php/RVM)

To use rvm, the user must be in the `rvm` group.

This module does this by default, but you have to do a login to be in the
group.

You can just invoke a login shell:

```sh
exec su -l "$USER"
```

TODO: make it respect rvm_path
TODO: solve the group restriction on rvm when installing to instantly use it
