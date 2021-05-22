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

## On Windows

From a POSIX compatibility layer `pont` is usable. To reduce the required
complexity of mixed os modules, `pont` by default does not run `*.sh`
scripts and only invokes `*.ps1` PowerShell scripts on these environments.
Stowing is also disabled as it's not available. You have to write custom
PowerShells scripts to have this functionality.

It also has to be run manually run from an elevated environment as it cannot
switch from inside (`sudo` is not available)

### Setup

First install [Git for Windows](https://git-scm.com/download/win).
You will need this to clone the `dotfiles` repository and it provides you a
`MINGW64` environment to run `pont` in.

> From administrator `Git Bash`

```sh
git clone --recurse-submodules -j8 \
  http://www.github.com/AlexAegis/dotfiles ~/AppData/Roaming/dotfiles
```

The default `DOTFILES_HOME` location is not applicable on windows so you must
override it. You can invoke `PowerShell` from `Git Bash`.
The path has to be defined as it appears from `Git Bash`.
The environment variable has to be system wide

> Still from administrator `Git Bash`

```sh
powershell "[Environment]::SetEnvironmentVariable(
  'DOTFILES_HOME',
  '$HOME/AppData/Roaming/dotfiles',
  [EnvironmentVariableTarget]::Machine)"
```

Then have pont setup itself:

```sh
~/AppData/Roaming/dotfiles/modules/pont/pont.sh pont
```

After this you can use `pont`. Installing the `windows` preset:

```sh
pont +windows
```
