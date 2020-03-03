# git

TODO: setup keyring

On WSL share with Host keyring

```bash
git config --global credential.helper "/mnt/c/Program\ Files/Git/mingw64/libexec/git-core/git-credential-manager.exe"
```

Askpass

```bash
git config --global core.askpass /usr/bin/ksshaskpass
```

TODO: check <https://github.com/oem/dotfiles/blob/master/gitconfig>
