#!/bin/sh

# deprecated, now it restarts itself
# systemctl enable --user entr-watch-watcher.service
# systemctl enable --user entr-watch-watcher.path
systemctl enable --user --now entr-watch.service
