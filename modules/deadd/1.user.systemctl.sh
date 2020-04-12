#!/bin/sh

systemctl enable --user --now deadd-notification-center.service
systemctl enable --user --now deadd-watcher.service
