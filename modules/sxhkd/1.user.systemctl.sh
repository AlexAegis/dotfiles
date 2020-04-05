#!/bin/sh

systemctl --user enable --now sxhkd.service
# Disabled in favour of the entr watcher, see readme for why
# systemctl --user enable --now sxhkd.path
