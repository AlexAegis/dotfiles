# dotfiles

My dotmodules™, and a live example of [dot](https://github.com/alexaegis/dot)

## [dot](./dot/readme.md)

## Install

```sh
git clone http://www.github.com/AlexAegis/dotfiles ~/.dotfiles
cd ~/.dotfiles/dot
git submodule update --init --recursive
sudo make # creates a symlink of ./dot.sh to /usr/local/bin
dot zsh vim # or just call ./dot.sh if you didn't `make`
```

## Uninstall

```sh
cd ~/.dotfiles/dot
sudo make clean # removes the symlink of ./dot.sh
```

TODO: Write about the concept of a junction module
check sys-\* modules for example

TODO: try to indroduce `~/.local/share/*` folders where locally installed
by using XDG_DATA_HOME
programs should be, not their config, those stay in .config
