#!/bin/sh

echo 'enable sxhkd'
systemctl enable --user --now sxhkd
echo 'enable sxhkd watcher service'
systemctl enable --user sxhkd-watcher.service
echo 'enable sxhkd watcher path'
systemctl enable --user --now sxhkd-watcher.path
