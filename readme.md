# dotfiles

My Dotfiles

Installing packages require root priviliges. Installing configs however don't. If you give `dot` root priviliges it will install all packages, but when not, it will only return a list of what it nee

## Prerequisites

The `base` module. See [Installation](#Installation).
This makes sure that `git` and `stow`, and some other packages are installed.

## Installation

```sh
git clone http://www.github.com/AlexAegis/dotfiles ~/.dotfiles
sudo -E ~/.dotfiles/dot install base
```

This will install [necessary packages](./modules/base/install.pacman.sh) and symlinks `dot` to `/usr/local/bin/dot`

## Usage

```sh
dot install module
```

TODO: Parameterize

## Goals

Use an interactive config
Be able to detect (by hostname) my pc (hardware based configs like audio)
Be able to detect distro (and use the correct installers)
And make a folder stucture to represent this

1. TMUX
2. SHELL
3. SSH and Security (PW protected primary, optional passwordless secondary)
   Option to use a global password in every later configuration step, or ask every time.

![Automation](https://imgs.xkcd.com/comics/automation.png)

Expectations:

- VERSIONING

```sh
tar --exclude='zsh/.tarhash' -c zsh | sha1sum > zsh/.tarhash
```

and save it in the repos module folder (so it doesn't conflict. It's gitignored anyways)
dont forget to ignore the tarhashfile itself from the hashing

- Should be environment aware

  > Should work on different type of distros
  > options to fail hard or force as much as it can

- Revertable changes

  > try using symlinks everywhere, collect
  > the installed symlinks in an uninstall script)

- Differentiate between being an owner and a guest. (Setup on remote)

- Provide update script for git based packages

- Dependency resolve:
  - check if dependency is fulfilled or not with unique script in the module
  - if not install that,

TODO Read: <http://www.gnu.org/savannah-checkouts/gnu/autoconf/manual/autoconf-2.69/autoconf.html#Portable-Shell>
