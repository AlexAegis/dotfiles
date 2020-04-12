#!/bin/sh

systemctl enable --user --now polybar.service
systemctl enable --user --now polybar-watcher.service
