# [Ruby](https://www.ruby-lang.org/en/)

> [Arch Wiki](https://wiki.archlinux.org/index.php/Ruby)

## [rbenv](https://github.com/rbenv/rbenv)

Simple ruby version switcher

### [Ruby Build](https://github.com/rbenv/ruby-build#installation)

As a plugin to rbenv, it can install ruby versions

### Rbenv doctor

This script will check your environment to determine if everything is set up
correctly or not.

```sh
curl -fsSL https://github.com/rbenv/rbenv-installer/raw/master/bin/rbenv-doctor | bash
```

## [RVM](https://wiki.archlinux.org/index.php/RVM)

> Switched to rbenv

To use rvm, the user must be in the `rvm` group.

This module does this by default, but you have to do a login to be in the
group.

You can just invoke a login shell:

```sh
exec su -l "$USER"
```
