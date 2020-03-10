# [dot.sh](./dot.sh)

[dot.sh](./dot.sh) and my dotfiles as a live example

## What is dot

It's a single shell script designed to install programs and personal configs
in bulk in a highly configurable and environmentally aware way using as simple
techniques as possible.

### It's my first shell script project so please leave issues if you have any

![Automation](https://imgs.xkcd.com/comics/automation.png)

## Prerequisites

A [POSIX](https://en.wikipedia.org/wiki/POSIX) capable shell on `/bin/sh`
to run `dot` itself.
I tested on `dash` and `bash`.

## Installation

```sh
git clone http://www.github.com/AlexAegis/dotfiles ~/.dotfiles
```

> Read what dotlink does before you execute it with sudo

Then to make `dot` available everywhere create a symlink of it with
[`dotlink`](./dotlink.sh)

```sh
sudo ~/.dotfiles/dotlink.sh
```

This will symlink `dot` to `/usr/local/bin/dot` so it's more convinient to use.

## Usage

```sh
dot [-] modules...
```

TODO: Parameterize

## Features

Dot

### Dotmodules

### Installi

## Priviliges

Installing packages require root priviliges. Installing configs however don't.
But you don't want to clutter your home folder with files owned by `root`.
That's why `dot` handles privileges based on the name of the module scripts.
A user priviliged script will always run with your user even if you run dot with
`sudo` and likewise a root priviliged script with always try to invoke sudo.

## Versioning

Already installed dotmodules will be only reinstalled if the module has changed
or when forced with `-f`. This is done by making a `sha1` checksum of the `tar`
of the folder and saving it into a file called `.tarhash` in the module.

# Zsh plugin

Contains autocompletions for [`dot`](https://github.com/AlexAegis/dotfiles)

It looks for modules in `$DOT_MODULES_FOLDER` which by default
is `$DOT_FOLDER/modules/*`.
And It looks for presets in `$DOT_PRESETS_FOLDER` which by default
is `$DOT_FOLDER/presets/*`.

And `$DOT_FOLDER` by default is `~/.dotfiles`.

If using [Antibody](https://github.com/getantibody/antibody) then add
this enty to your plugin file to get `dot` autocompletions:

```sh
AlexAegis/dotfiles path:plugins/dot
```
