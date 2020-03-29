# [pass](https://wiki.archlinux.org/index.php/Pass)

> <https://www.passwordstore.org/>

I sync my passwords to a private GitHub repository.
The only thing I have to handle manually is the gpg keys.

> If this dotmodule is installed then these utilities are already on path

TODO: sudo exclude pass from NOPASSWD
TODO: setup browserpass

## Creating keys

Use multiline to store username
First line is always the password, the second one is the username

```sh
pass insert --multiline
```

## Reinitialize Pass

The entire password-store can be reinitialized using a new gpg key if the old
one is present.

```sh
pass-reinit
# pass-export-key
```

This won't delete passwords but it creates a new key and rewrites each
password to use this new password. Don't forget to export it and backup it!

## Truncate history

When this module updates it automatically synchronizes with the remote
and [truncates the history](./.pass/bin/pass-truncate-history), reducing the
history to just one commit. So if you delete a password, it's gone for good.

```sh
pass-truncate-history
```

## Export secret

To export the currently used `gpg` key use
[`pass-export-key`](./.pass/bin/pass-export-key)
Then copy it somewhere secure.

```sh
pass-export-key
```

## Import secret

To import just run [`pass-import-key`](./.pass/bin/pass-import-key)

```sh
pass-import-key
```

TODO: SET PASSWORD_STORE_DIR in env
