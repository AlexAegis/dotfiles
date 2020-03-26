#!/bin/sh

echo 'Daemon reload'
systemctl --user daemon-reload
echo 'Restart sxhkd'
systemctl restart --user sxhkd
echo 'Restart sxhkd-watcher'
systemctl restart --user sxhkd-watcher.service
systemctl restart --user sxhkd-watcher.path
