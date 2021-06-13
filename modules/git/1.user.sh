#!/bin/sh

# The dotfiles repository is intented to be the very first thing you
# clone onto a new system so having ssh set up is not expected, so at that
# point cloning from git@github.com would be cumbersome
# But for the repository to be easily editable, ssh is necessary
# and this script automatically changes all mentions of the http link to the
# ssh one.
./XDG_BIN_HOME.git/github_origin_toggle ../../ ssh
./XDG_BIN_HOME.git/github_origin_toggle ../pont/ ssh
