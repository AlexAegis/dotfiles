# dotfiles

[![Lint](https://github.com/AlexAegis/dotfiles/workflows/Lint/badge.svg)](https://github.com/AlexAegis/dotfiles/actions?query=workflow%3ALint)

My dotmodules™, and a live example of
[pont](https://github.com/alexaegis/pont), my dotmodule manager.

[![my_stuff](./docs/images/behold_dotfiles.jpg)](./modules)

## Install

This dotfiles repository is set up so `pont` is used as a module itself.
(Which is not necessary to use `pont`)

This will set up itself:

```sh
git clone --recurse-submodules -j8 \
  http://www.github.com/AlexAegis/dotfiles ~/.config/dotfiles
~/.config/dotfiles/modules/pont/pont.sh pont
```

If you use different folders than the default, for example, you want to
put your dotfiles to `~/.dotfiles` instead of the default `~/.config/dotfiles`
then before installing anything, set the `DOTFILES_HOME` variable beforehand.

```sh
export DOTFILES_HOME="$HOME/.dotfiles"
~/.dotfiles/modules/pont/pont.sh pont
```

Make sure pont's configs are available when you use them, it can't bootstrap
itself if the !

Then install whatever you'd like. For example, to bootstrap an arch system:

```sh
pont +arch
```
