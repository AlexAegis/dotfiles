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
  https://www.github.com/AlexAegis/dotfiles ~/.config/dotfiles
~/.config/dotfiles/modules/pont/pont.sh pont
```

Then install whatever you'd like. For example, to bootstrap an arch system:

```sh
pont +arch
```

### On macOS

For macOS the `XDG` variables are set around the `~/Library` folder.

```sh
git clone --recurse-submodules -j8 \
  https://www.github.com/AlexAegis/dotfiles ~/Library/Preferences/dotfiles
~/Library/Preferences/dotfiles/modules/pont/pont.sh pont
```
