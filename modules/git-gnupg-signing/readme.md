# Git gnupg signing

This optional module adds configuration for git to tell that it should
sign commit, and how should it do it. I don't want to have them signed
everywhere so it's configured in a separate module.

## Windows

Also define the gpg program. (You can install `gnupg` using `choco`)

```ps1
git config --global gpg.program "C:\Program Files (x86)\GnuPG\bin\gpg.exe"
```

Don't forget to adjust the id of the imported secret key.

Inspect it using `gpg --list-secret-keys --keyid-format LONG`. It's the part
after `rsa4096`.

## Indirect dependency

Since git doesn't really support config directories or globbed config imports
all possible git config files have to be explicitly mentioned in the main
git config file. Thankfully it does not throw a tantrum for non-existent
files.

TODO: The gitconfig file should be created dynamically as the key handle can be
different
