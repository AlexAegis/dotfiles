# dotfiles

My dotmodules™, and a live example of [dot](https://github.com/alexaegis/dot)
my dotmodule manager.

[![my_stuff](./docs/images/behold_dotfiles.jpg)](https://imgflip.com/i/40ubdj?herp=1589096423404)

## Install

This dotfiles repository is set up so `dot` is used as a module itself.
(Which is not necessary to use `dot`)

This will set up itself:

```sh
git clone --recurse-submodules -j8 \
  http://www.github.com/AlexAegis/dotfiles ~/.config/dotfiles
~/.config/dotfiles/modules/dot/dot.sh dot
```

If you use different folders than the default, for example, you want to
put your dotfiles to `~/.dotfiles` instead of the default `~/.config/dotfiles`
then before installing anything, set the `DOTFILES_HOME` variable beforehand.

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
