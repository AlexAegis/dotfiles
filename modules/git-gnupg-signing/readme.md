# Git gnupg signing

This optional module adds configuration for git to tell that it should
sign commit, and how should it do it. I don't want to have them signed
everywhere so it's configured in a separate module.

## Indirect dependency

Since git doesn't really support config directories or globbed config imports
all possible git config files have to be explicitly mentioned in the main
git config file. Thankfully it does not throw a tantrum for non-existent
files.

TODO: The gitconfig file should be created dynamically as the key handle can be
different
