# [GnuPG](https://gnupg.org/)

> [Arch Wiki](https://wiki.archlinux.org/index.php/GnuPG)\
> [Gentoo Wiki](https://wiki.gentoo.org/wiki/GnuPG)

This module sets up the `gpg` environment, but doesn't generate anything
on it's own.

## PAM

Automatic pam unlock handled by
[pam-gnupg](https://github.com/cruegge/pam-gnupg)

### SSH

You can add your ssh keys to the gnupg keys with `ssh-add`. It's pre-requisites
are having the `SSH_AUTH_SOCK` variable set, and the `ssh-agent` running.

You only have to do this once for your ssh key.

## Reloading the config

```sh
gpg-connect-agent reloadagent /bye
```

## Trust

To edit the trust of keys

```sh
gpg --edit-key <id>
gpg> trust
```

## ASCII Armor

```sh
gpg --list-secret-keys --keyid-format LONG
```

Then using the id in the `ssb` line under your key
(`ssb rsa4096/<this-part> 2021-01-01 [SEA]`), you can generate an armored
public key block.

```sh
gpg --armor --export <ssb-id>
```

You can then add this to certain services you want to authenticate with, like
GitHub.

> Tip: If you store these in a file, use the extension `.asc`, like
> `id.pub.asc`

## TTY

You can force gnupg to use a certain tty when asking for a password with an
environmental variable:

```sh
# Force gnupg to use the same tty where the login happened
# (If this is loaded on login)
export GPG_TTY=$(tty)
gpg-connect-agent updatestartuptty /bye >/dev/null
```
