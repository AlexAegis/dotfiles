# dotfiles

My Dotfiles

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

- Should be environment aware

  > Should work on different type of distros
  > options to fail hard or force as much as it can

- Revertable changes

  > try using symlinks everywhere, collect
  > the installed symlinks in an uninstall script)

- Differentiate between being an owner and a guest. (Setup on remote)

- Provide update script for git based packages

TODO Read: http://www.gnu.org/savannah-checkouts/gnu/autoconf/manual/autoconf-2.69/autoconf.html#Portable-Shell
