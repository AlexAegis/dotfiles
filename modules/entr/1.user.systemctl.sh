#!/bin/sh

systemctl enable --user --now entr-watch
systemctl enable --user --now entr-watch-watcher.service
systemctl enable --user --now entr-watch-watcher.path
