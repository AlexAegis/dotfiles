# Git gnupg signing

This optional module adds configuration for git to tell that it should
sign commit, and how should it do it. I don't want to have them signed
everywhere so it's configured in a separate module.

# Changing the key

List your gpg keys:

> Example taken from [GitHub](https://docs.github.com/en/authentication/managing-commit-signature-verification/telling-git-about-your-signing-key)

```sh
$ gpg --list-secret-keys --keyid-format=long

sec   4096R/3AA5C34371567BD2 2016-03-10 [expires: 2017-03-10]
uid                          Hubot <hubot@example.com>
ssb   4096R/4BB6D45482678BE3 2016-03-10
```

And use the id in the sec row, here it's: `3AA5C34371567BD2`

## Re-signing earlier commits

For example, re-signing the last 4 commits of the current branch:

> **If you do this in a dotfiles repo like this one**, where the global git config is
> stored and `$XDG_CONFIG_HOME/git/config` just links into this repo, then make sure
> to set your target signing key locally for this repo before doing this using
> `git config user.signingkey NEW-GPG-ID`. If you don't, you will have problems:
> If you're lucky, and that old key from an older commit is no longer on
> your keychain, the rebase will just simply stop with an error. If you still have
> the old keys on your keychain, it will happily apply the key inside the commit
> to the commit. Leading to a whole lot of confusion.

```sh
git rebase --exec 'git commit --amend --no-edit -n -S' -i HEAD~9
```

Verify a commits signature:

```sh
git verify-commit <commit-hash>
```

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
