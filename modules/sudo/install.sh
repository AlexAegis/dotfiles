#!/bin/sh

# Dependencies: sudo, vi

# TODO Copy/Append the example file to the real sudoers file !!! with visudo !!!

# echo "$USER ALL=NOPASSWD:/usr/bin/rsync" |
# (sudo su -c 'EDITOR="tee" visudo -f /etc/sudoers.d/rsync')

# TODO: instead of editing sudoers (cehck it something might have to be deleted)
# TODO: write (DO NOT SYMLINK) new files to /etc/sudoers.d/ with v
# visudo -c -q -f filename


# TODO: %wheel ALL=(ALL) NOPASSWD: ALL ?

# https://stackoverflow.com/questions/323957
