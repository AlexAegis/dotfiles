# [pass](https://wiki.archlinux.org/index.php/Pass)

I sync my passwords to a private GitHub repository.
The only thing I have to handle manually is the gpg keys.

The entire password-store can be reinitialized using a new gpg key.

> If this dotmodule is installed then these utilities are already on path

## Export secret

To export the currently used `gpg` key use
[`pass-export-key`](./.pass/bin/pass-export-key)
Then copy it somewhere secure.

## Import secret

To import just run [`pass-import-key`](./.pass/bin/pass-import-key)
gpg --import my-private-key.asc

```sh

```

TODO:
question: does pass keep history? If yes its not secure, old
compromized keys can be used for older passwords

## T

Use multiline to store username
First line is always the password, the second one is the username

```sh
pass insert --multiline
```

TODO: fix sudoers that pass always require sudo and password? Investigate
