# git

This folder is called git-scm to avoid having a `.git` folder to stow

On WSL share with Host keyring

```bash
git config --global credential.helper "/mnt/c/Program\ Files/Git/mingw64/\
libexec/git-core/git-credential-manager.exe"
```

To see which config comes from where

```sh
git config --list --show-origin
```

or if this module is installed just use the git alias of:

```sh
git lc
```

> **l**ist **c**onfig
