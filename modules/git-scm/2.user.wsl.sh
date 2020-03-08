#!/bin/sh

# TODO: Make sure this runs when using wsl
# TODO: My idea is to (if there is no easy way of determining
# TODO: whether or not you're in a wsl instance and what kind)
# TODO: have modules that simply put stuff to bin so command -v recognizes them
git config --global credential.helper "/mnt/c/Program\ Files/Git/mingw64/\
libexec/git-core/git-credential-manager.exe"
