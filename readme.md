# [dot.sh](./dot.sh)

[dot.sh](./dot.sh) and my dotfiles as a live example

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
