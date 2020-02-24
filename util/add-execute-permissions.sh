#!/bin/sh
# From: https://askubuntu.com/questions/889344/command-to-perform-a-recursive-chmod-to-make-all-sh-files-within-a-directory-ex
# Adds execute rights to every .sh file under this
# chmod +x !!
find ./ -type f -iname "*.sh" -exec chmod +x {} \;

# TODO: Expand to .fish and .ysh
