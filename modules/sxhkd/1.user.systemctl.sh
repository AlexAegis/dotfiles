#!/bin/sh

systemctl --user enable --now sxhkd.service
systemctl --user enable --now sxhkd-watcher.service
# Disabled in favour of the entr watcher, see readme for why
# systemctl --user enable --now sxhkd.path
