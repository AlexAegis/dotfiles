# dotfiles

My dotmodules™, and a live example of [dot](https://github.com/alexaegis/dot)

## Install

This dotfiles repo is set up so `dot` is used as a module itself. (Which is
not needed)

This will set up itself, putting dot to the PATH:

```sh
git clone --recurse-submodules -j8 \
  http://www.github.com/AlexAegis/dotfiles ~/.config/dotfiles
~/.config/dotfiles/modules/dot/dot.sh dot
```

If you use different folders than the default, for example, you want to
put your dotfiles to `~/.dotfiles` instead of the default `~/.config/dotfiles`
then before installing anything, set the `DOTFILES_HOME` variable manually.

```sh
export DOTFILES_HOME="$HOME/.dotfiles"
~/.dotfiles/modules/dot/dot.sh dot
```

Make sure dots configs are available when you use them, it can't bootstrap
itself if the !

Then install whatever you'd like. For example, to bootstrap an arch system:

```sh
dot +arch
```

## Notes

TODO: Write about the concept of a junction module
check sys-\* modules for example

TODO: try to indroduce `~/.local/share/*` folders where locally installed
by using XDG_DATA_HOME
programs should be, not their config, those stay in .config
