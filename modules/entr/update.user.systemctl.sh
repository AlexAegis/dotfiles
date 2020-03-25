#!/bin/sh

systemctl --user daemon-reload
systemctl restart --user entr-watch
systemctl restart --user entr-watch-watcher.service
systemctl restart --user entr-watch-watcher.path
