#!/bin/sh

systemctl --user enable --now sxhkd.service
systemctl --user enable --now sxhkd-watcher.service
systemctl --user enable --now sxhkd-environment-watcher.service
