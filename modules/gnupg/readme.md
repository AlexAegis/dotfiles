# [GnuPG](https://gnupg.org/)

> [Arch Wiki](https://wiki.archlinux.org/index.php/GnuPG)

This module sets up the `gpg` environment, but doesn't generate anything
on it's own.

## TTY

You can force gnupg to use a certain tty when asking for a password with an
environmental variable:

```sh
# Force gnupg to use the same tty where the login happened
# (If this is loaded on login)
export GPG_TTY=$(tty)
gpg-connect-agent updatestartuptty /bye >/dev/null
```
