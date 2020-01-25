#!/bin/bash
# From: https://askubuntu.com/questions/889344/command-to-perform-a-recursive-chmod-to-make-all-sh-files-within-a-directory-ex
# chmod +x !!
find ./ -type f -iname "*.sh" -exec chmod +x {} \;
